"""💬️ Inspect BIM issue exports through ZIP, lxml and NumPy.

Run standalone:

    python 🐍️.py check <path to 🧫️fixtures/🚪️bcf/🏢️frame/issues.bcf>     # exit 1 on any disagreement
"""

# region 🔖️Imports
import json
import re
import sys
import zipfile
from pathlib import Path

import numpy as np
from lxml import etree

# endregion 🔖️Imports

UUID = re.compile(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
IFC_GUID = re.compile(r"^[0-9A-Za-z_$]{22}$")
DATE_TIME = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})?$")
TOPIC_ORDER = ["ReferenceLink", "Title", "Priority", "Index", "Labels", "CreationDate", "CreationAuthor", "ModifiedDate", "ModifiedAuthor", "DueDate", "AssignedTo", "Stage", "Description", "BimSnippet", "DocumentReference", "RelatedTopic"]


def vector(node):
    return [float(node.findtext(axis)) for axis in "XYZ"]


# region 🔖️Container
def read(container):
    """📦️ The entries of the container by name, the CRC of every entry verified."""
    with zipfile.ZipFile(container) as archive:
        broken = archive.testzip()
        if broken:
            raise ValueError("corrupt entry %s" % broken)
        return {name: archive.read(name) for name in archive.namelist()}


def report(container):
    """💬️ The table of the topics of a container and the problems against the BCF 2.1 rules: `(table, problems)`."""
    files, problems, table = read(container), [], {}
    version = etree.fromstring(files["bcf.version"]) if "bcf.version" in files else None
    if version is None or version.get("VersionId") != "2.1":
        problems.append("bcf.version must declare VersionId 2.1")
    for topic_guid in sorted({name.split("/")[0] for name in files if name.endswith("/markup.bcf")}):
        if not UUID.match(topic_guid):
            problems.append("topic folder %s is no UUID" % topic_guid)
        markup = etree.fromstring(files["%s/markup.bcf" % topic_guid])
        topic = markup.find("Topic")
        if markup.tag != "Markup" or markup[0].tag != "Header" or topic is None:
            problems.append("%s: markup needs Header and Topic" % topic_guid)
            continue
        if topic.get("Guid") != topic_guid or not topic.get("TopicStatus"):
            problems.append("%s: Topic needs its Guid and a TopicStatus" % topic_guid)
        order = [TOPIC_ORDER.index(child.tag) for child in topic if child.tag in TOPIC_ORDER]
        if order != sorted(order):
            problems.append("%s: Topic children out of the markup.xsd order: %s" % (topic_guid, [child.tag for child in topic]))
        for required in ("Title", "CreationDate", "CreationAuthor"):
            if topic.find(required) is None:
                problems.append("%s: Topic lacks %s" % (topic_guid, required))
        if not DATE_TIME.match(topic.findtext("CreationDate", "")):
            problems.append("%s: CreationDate %r is no xs:dateTime" % (topic_guid, topic.findtext("CreationDate")))
        comments = []
        for comment in markup.findall("Comment"):
            if not UUID.match(comment.get("Guid", "")) or not DATE_TIME.match(comment.findtext("Date", "")):
                problems.append("%s: a comment needs a UUID Guid and an xs:dateTime Date" % topic_guid)
            comments.append({"author": comment.findtext("Author"), "text": comment.findtext("Comment")})
        row = {
            "status": topic.get("TopicStatus"),
            "priority": topic.findtext("Priority"),
            "labels": [label.text for label in topic.findall("Labels")],
            "author": topic.findtext("CreationAuthor"),
            "assignee": topic.findtext("AssignedTo", ""),
            "section": None,
            "comments": comments,
            "camera": None,
            "selection": [],
            "isolate": [],
        }
        for reference in markup.iter("Viewpoints"):
            path = "%s/%s.bcfv" % (topic_guid, reference.get("Guid"))
            if reference.findtext("Viewpoint") != reference.get("Guid", "") + ".bcfv" or path not in files:
                problems.append("%s: viewpoint %s is missing" % (topic_guid, path))
                continue
            info = etree.fromstring(files[path])
            if info.tag != "VisualizationInfo" or info.get("Guid") != reference.get("Guid"):
                problems.append("%s: viewpoint identity differs from its reference" % topic_guid)
            planes = info.findall("ClippingPlanes/ClippingPlane")
            if planes:
                faces = {}
                for plane in planes:
                    normal = vector(plane.find("Direction"))
                    location = vector(plane.find("Location"))
                    for axis in range(3):
                        for sign in (-1, 1):
                            if normal == [sign if i == axis else 0 for i in range(3)]:
                                faces[axis, sign] = location[axis]
                if len(planes) == len(faces) == 6:
                    row["section"] = {side: [faces[i, sign] for i in range(3)] for side, sign in (("min", -1), ("max", 1))}
                else:
                    problems.append("%s: six outward-facing clipping planes expected" % topic_guid)
            camera = info.find("PerspectiveCamera")
            if camera is not None:
                eye, look, up = vector(camera.find("CameraViewPoint")), vector(camera.find("CameraDirection")), vector(camera.find("CameraUpVector"))
                if abs(np.linalg.norm(look) - 1) > 1e-9 or abs(np.linalg.norm(up) - 1) > 1e-9 or abs(float(np.dot(look, up))) > 1e-9:
                    problems.append("%s: direction and up vector must be orthonormal" % topic_guid)
                row["camera"] = {"eye": eye, "look": look, "up": up}
            row["selection"] += [node.get("IfcGuid") for node in info.iter("Component") if node.getparent().tag == "Selection"]
            visibility = info.find(".//Visibility")
            if visibility is not None and visibility.get("DefaultVisibility") == "false":
                row["isolate"] += [node.get("IfcGuid") for node in visibility.iter("Component")]
        row["selection"], row["isolate"] = sorted(row["selection"]), sorted(row["isolate"])
        if any(not IFC_GUID.match(guid) for guid in row["selection"] + row["isolate"]):
            problems.append("%s: a component is no IFC GlobalId" % topic_guid)
        table[topic.findtext("Title")] = row
    return table, problems


# endregion 🔖️Container


# region 🔖️Handlers
def container_handler(ctx):
    """🧭️ Oracle handler: what the committed container says (registered in the ORACLE role only)."""
    from semio_repo_test import Outcome

    table, problems = report(__import__("io").BytesIO(ctx.input_bytes(next(uri for uri in ctx.step_input_uris() if uri.endswith(".bcf")))))
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(table)


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-bcf-frame", container_handler)


# endregion 🔖️Handlers


def main(arguments):
    command, container = arguments[0], Path(arguments[1])
    table, problems = report(container)
    if command == "report":
        print(json.dumps(table, indent=2, ensure_ascii=False))
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s (lxml %s, numpy %s, %d topics)" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees", etree.LXML_VERSION, np.__version__, len(table)))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
