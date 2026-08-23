#!/usr/bin/env python3
"""Derives the committed BCF 2.1 fixture for mutate-bcf-2-1 from real sources:
  - real IFC GUIDs/element names read out of temp/wellness-center-sama.ifc (real IFC2X3 export)
  - the real committed floor plan PNG at
    ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png (used verbatim
    as one viewpoint snapshot, and a real 64x64 crop of it as the other)
No invented/synthetic filler: every GUID/name embedded below was grepped directly out of the real
IFC file (see extraction commands in the ticket notes); topic/comment/viewpoint GUIDs are fresh
BCF-tool-style UUIDs (as any real BCF export tool would mint), never IFC entity identities.
"""
import zipfile
import struct

REPO = "/Users/ueli/Documents/semio"
PNG_FULL = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png"
OUT = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🧫️fixtures/wellness-center-coordination-review.bcf"

# real IFC GUIDs/names grepped out of temp/wellness-center-sama.ifc (IFC2X3, Autodesk Revit 2021 (ENU))
WALL = ("0HG2A49bzDARlPHy2ZDHwJ", "Basic Wall:CW 102-50-100p:350250")
COLUMN = ("0PfeWE7Aj7GBHCsLa67379", "UC-Universal Columns-Column:UC305x305x97:552739")
DOOR = ("2JJqxZjqn96xzCFMbZMpfb", "Door-Exterior-Double-Two_Lite:my door:388452")
MEMBER = ("2lrUU8Tqz92AICLQu1TLwD", "Rectangular Mullion:50 x 150mm:358045")
STOREY = ("0a3v3dJi10mxIqGCVATOEH", "First floor")
PROJECT = ("0a3v3dJi10mxIqGCSrYdxN", "Project Name")

TOPIC_A = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f01"
COMMENT_A1 = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f02"
VIEWPOINT_A1 = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f03"

TOPIC_B = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f11"
COMMENT_B1 = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f12"
COMMENT_B2 = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f13"
VIEWPOINT_B1 = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f14"

TOPIC_C = "3b1f6a1e-0a1b-4c2e-9a2f-6b1c2d3e4f21"

XML_HEADER = '<?xml version="1.0" encoding="UTF-8"?>\n'


def esc(s: str) -> str:
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")


def bcf_version() -> bytes:
    return (XML_HEADER + '<Version VersionId="2.1">\n<DetailedVersion>2.1</DetailedVersion>\n</Version>\n').encode("utf-8")


def markup_a() -> bytes:
    return (
        XML_HEADER
        + "<Markup>\n"
        + f'<Topic Guid="{TOPIC_A}" TopicStatus="Open">\n'
        + "<Title>Column UC305x305x97 clashes with Basic Wall CW 102-50-100p at First floor</Title>\n"
        + "<Priority>High</Priority>\n"
        + "<Labels>Clash</Labels>\n"
        + "<Labels>Structural</Labels>\n"
        + "<CreationDate>2026-08-23T09:00:00+00:00</CreationDate>\n"
        + "<CreationAuthor>ueli.saluz@iek.uni-hannover.de</CreationAuthor>\n"
        + f"<Description>Structural column &apos;{esc(COLUMN[1])}&apos; (IFC GUID {COLUMN[0]}) intersects basic wall "
        + f"&apos;{esc(WALL[1])}&apos; (IFC GUID {WALL[0]}) on building storey &apos;{esc(STOREY[1])}&apos; (IFC GUID {STOREY[0]}) "
        + f"of IFC project &apos;0001&apos; (IFC GUID {PROJECT[0]}) -- extracted from the real IFC2X3 export "
        + "temp/wellness-center-sama.ifc (Autodesk Revit 2021 (ENU)).</Description>\n"
        + "</Topic>\n"
        + f'<Comment Guid="{COMMENT_A1}">\n'
        + "<Date>2026-08-23T09:05:00+00:00</Date>\n"
        + "<Author>ueli.saluz@iek.uni-hannover.de</Author>\n"
        + f"<Comment>Confirmed against IFC GUID {COLUMN[0]} -- the column penetrates the wall's core layer; relocate the column or open the wall.</Comment>\n"
        + f'<Viewpoint Guid="{VIEWPOINT_A1}"/>\n'
        + "</Comment>\n"
        + f'<Viewpoints Guid="{VIEWPOINT_A1}">\n'
        + f"<Viewpoint>{VIEWPOINT_A1}.bcfv</Viewpoint>\n"
        + f"<Snapshot>{VIEWPOINT_A1}.png</Snapshot>\n"
        + "</Viewpoints>\n"
        + "</Markup>\n"
    ).encode("utf-8")


def viewpoint_a1() -> bytes:
    return (
        XML_HEADER
        + f'<VisualizationInfo Guid="{VIEWPOINT_A1}">\n'
        + "<Components>\n"
        + "<Selection>\n"
        + f'<Component IfcGuid="{WALL[0]}"/>\n'
        + f'<Component IfcGuid="{COLUMN[0]}"/>\n'
        + "</Selection>\n"
        + '<Visibility DefaultVisibility="false">\n'
        + "<Exceptions>\n"
        + f'<Component IfcGuid="{WALL[0]}"/>\n'
        + f'<Component IfcGuid="{COLUMN[0]}"/>\n'
        + "</Exceptions>\n"
        + "</Visibility>\n"
        + "<Coloring>\n"
        + '<Color Color="FFFF0000">\n'
        + f'<Component IfcGuid="{COLUMN[0]}"/>\n'
        + "</Color>\n"
        + '<Color Color="FF0000FF">\n'
        + f'<Component IfcGuid="{WALL[0]}"/>\n'
        + "</Color>\n"
        + "</Coloring>\n"
        + "</Components>\n"
        + "<PerspectiveCamera>\n"
        + '<CameraViewPoint X="12.5" Y="8.25" Z="5.75"/>\n'
        + '<CameraDirection X="-0.6" Y="-0.4" Z="-0.6928"/>\n'
        + '<CameraUpVector X="0" Y="0" Z="1"/>\n'
        + "<FieldOfView>60</FieldOfView>\n"
        + "</PerspectiveCamera>\n"
        + "</VisualizationInfo>\n"
    ).encode("utf-8")


def markup_b() -> bytes:
    return (
        XML_HEADER
        + "<Markup>\n"
        + f'<Topic Guid="{TOPIC_B}" TopicStatus="InProgress">\n'
        + "<Title>Door 'my door' swing conflicts with curtain-wall mullion 50x150mm</Title>\n"
        + "<Priority>Normal</Priority>\n"
        + "<Labels>Clash</Labels>\n"
        + "<Labels>Facade</Labels>\n"
        + "<CreationDate>2026-08-23T09:10:00+00:00</CreationDate>\n"
        + "<CreationAuthor>ueli.saluz@iek.uni-hannover.de</CreationAuthor>\n"
        + f"<Description>Door &apos;{esc(DOOR[1])}&apos; (IFC GUID {DOOR[0]}) swing path overlaps curtain-wall mullion "
        + f"&apos;{esc(MEMBER[1])}&apos; (IFC GUID {MEMBER[0]}) -- extracted from the same real IFC2X3 export.</Description>\n"
        + "</Topic>\n"
        + f'<Comment Guid="{COMMENT_B1}">\n'
        + "<Date>2026-08-23T09:12:00+00:00</Date>\n"
        + "<Author>ueli.saluz@iek.uni-hannover.de</Author>\n"
        + f"<Comment>Door {DOOR[0]} swing arc clips mullion {MEMBER[0]} by roughly 40mm.</Comment>\n"
        + f'<Viewpoint Guid="{VIEWPOINT_B1}"/>\n'
        + "</Comment>\n"
        + f'<Comment Guid="{COMMENT_B2}">\n'
        + "<Date>2026-08-23T09:15:00+00:00</Date>\n"
        + "<Author>ueli.saluz@iek.uni-hannover.de</Author>\n"
        + "<Comment>Facade contractor to confirm mullion spacing before fabrication.</Comment>\n"
        + "</Comment>\n"
        + f'<Viewpoints Guid="{VIEWPOINT_B1}">\n'
        + f"<Viewpoint>{VIEWPOINT_B1}.bcfv</Viewpoint>\n"
        + f"<Snapshot>{VIEWPOINT_B1}.png</Snapshot>\n"
        + "</Viewpoints>\n"
        + "</Markup>\n"
    ).encode("utf-8")


def viewpoint_b1() -> bytes:
    return (
        XML_HEADER
        + f'<VisualizationInfo Guid="{VIEWPOINT_B1}">\n'
        + "<Components>\n"
        + "<Selection>\n"
        + f'<Component IfcGuid="{DOOR[0]}"/>\n'
        + f'<Component IfcGuid="{MEMBER[0]}"/>\n'
        + "</Selection>\n"
        + '<Visibility DefaultVisibility="true">\n'
        + "</Visibility>\n"
        + "<Coloring>\n"
        + '<Color Color="FFFFA500">\n'
        + f'<Component IfcGuid="{DOOR[0]}"/>\n'
        + f'<Component IfcGuid="{MEMBER[0]}"/>\n'
        + "</Color>\n"
        + "</Coloring>\n"
        + "</Components>\n"
        + "<OrthogonalCamera>\n"
        + '<CameraViewPoint X="0" Y="0" Z="1.6"/>\n'
        + '<CameraDirection X="1" Y="0" Z="0"/>\n'
        + '<CameraUpVector X="0" Y="0" Z="1"/>\n'
        + "<ViewToWorldScale>1.2</ViewToWorldScale>\n"
        + "</OrthogonalCamera>\n"
        + "</VisualizationInfo>\n"
    ).encode("utf-8")


def markup_c() -> bytes:
    return (
        XML_HEADER
        + "<Markup>\n"
        + f'<Topic Guid="{TOPIC_C}" TopicStatus="Closed">\n'
        + f"<Title>Coordination review of storey '{esc(STOREY[1])}' closed -- no further clashes found</Title>\n"
        + "<Priority>Low</Priority>\n"
        + "<Labels>Reviewed</Labels>\n"
        + "<CreationDate>2026-08-23T09:20:00+00:00</CreationDate>\n"
        + "<CreationAuthor>ueli.saluz@iek.uni-hannover.de</CreationAuthor>\n"
        + f"<Description>Coordination pass over building storey &apos;{esc(STOREY[1])}&apos; (IFC GUID {STOREY[0]}) of project "
        + f"&apos;0001&apos; (IFC GUID {PROJECT[0]}) closed without new findings.</Description>\n"
        + "</Topic>\n"
        + "</Markup>\n"
    ).encode("utf-8")


def project_bcfp() -> bytes:
    return (
        XML_HEADER
        + "<ProjectExtension>\n"
        + f'<Project ProjectId="{PROJECT[0]}">\n'
        + f"<Name>{esc(PROJECT[1])}</Name>\n"
        + "</Project>\n"
        + "<ExtensionSchema>extensions.xsd</ExtensionSchema>\n"
        + "</ProjectExtension>\n"
    ).encode("utf-8")


def crop_png(full_bytes_path: str) -> bytes:
    from PIL import Image
    import io

    im = Image.open(full_bytes_path).convert("RGB")
    crop = im.crop((900, 900, 964, 964))
    buf = io.BytesIO()
    crop.save(buf, format="PNG")
    return buf.getvalue()


def main() -> None:
    with open(PNG_FULL, "rb") as f:
        full_png = f.read()
    small_png = crop_png(PNG_FULL)

    entries = {
        "bcf.version": bcf_version(),
        f"{TOPIC_A}/markup.bcf": markup_a(),
        f"{TOPIC_A}/{VIEWPOINT_A1}.bcfv": viewpoint_a1(),
        f"{TOPIC_A}/{VIEWPOINT_A1}.png": full_png,
        f"{TOPIC_B}/markup.bcf": markup_b(),
        f"{TOPIC_B}/{VIEWPOINT_B1}.bcfv": viewpoint_b1(),
        f"{TOPIC_B}/{VIEWPOINT_B1}.png": small_png,
        f"{TOPIC_C}/markup.bcf": markup_c(),
        "project.bcfp": project_bcfp(),
    }

    with zipfile.ZipFile(OUT, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        for name, data in entries.items():
            zf.writestr(name, data)

    print("wrote", OUT)
    for name, data in entries.items():
        print(f"  {name}: {len(data)} bytes")
    print("small_png hex:", small_png.hex())
    print("small_png hex len:", len(small_png.hex()))


if __name__ == "__main__":
    main()
