#!/usr/bin/env python3
"""Derive a small, genuine PPTX subset (7 real slides + closed relationship graph) from the
real 2020 conference deck at temp/domai-specific-programmaning-language-for-architects.pptx.
Stdlib-only (zipfile/re), no external deps -- this is a one-time ticket-folder derivation
script, not part of the repository's runtime."""
import re
import zipfile
import shutil

SRC = "/Users/ueli/Documents/semio/temp/domai-specific-programmaning-language-for-architects.pptx"
DST = "/private/tmp/claude-501/-Users-ueli-Documents-semio/34f3999f-e145-4d4e-ab13-c3c2aef22ddf/scratchpad/pptx-derive/semio-talk.pptx"

# Presentation-order slide files kept: slide1..slide6 (opening/title/agenda run of real
# text-only content) + slide23 ("Diagrammnotation", the first real Picture-bearing slide).
KEPT_SLIDE_NUMS = [1, 2, 3, 4, 5, 6, 23]
KEPT_SLIDE_FILES = {f"ppt/slides/slide{n}.xml" for n in KEPT_SLIDE_NUMS}

with zipfile.ZipFile(SRC) as zin:
    names = zin.namelist()
    read = lambda n: zin.read(n)

    pres_xml = read("ppt/presentation.xml").decode("utf-8")
    pres_rels = read("ppt/_rels/presentation.xml.rels").decode("utf-8")

    # rId -> slideN.xml for every slide relationship
    rid_to_slide = dict(re.findall(r'Id="(rId\d+)"[^>]*Target="slides/(slide\d+\.xml)"', pres_rels))
    slide_to_rid = {v: k for k, v in rid_to_slide.items()}
    kept_rids = {slide_to_rid[f"slide{n}.xml"] for n in KEPT_SLIDE_NUMS}

    # Trim <p:sldIdLst> to only the kept rIds, preserving original relative order.
    sldidlst_re = re.compile(r"<p:sldIdLst>.*?</p:sldIdLst>", re.S)
    original_block = sldidlst_re.search(pres_xml).group(0)
    entries = re.findall(r'<p:sldId id="(\d+)" r:id="(rId\d+)"/>', original_block)
    kept_entries = [(sid, rid) for sid, rid in entries if rid in kept_rids]
    assert len(kept_entries) == len(KEPT_SLIDE_NUMS), (len(kept_entries), kept_entries)
    new_block = "<p:sldIdLst>" + "".join(f'<p:sldId id="{sid}" r:id="{rid}"/>' for sid, rid in kept_entries) + "</p:sldIdLst>"
    new_pres_xml = pres_xml.replace(original_block, new_block)
    # Strip the PowerPoint-only <p:extLst> (p14 "sections" + p15 slide guides): a cosmetic
    # extension outside ECMA-376's own sldIdLst structure, not parsed by this subset's decoder,
    # and its p14:sldId entries reference numeric ids of slides this derivation drops -- removed
    # rather than left dangling.
    new_pres_xml = re.sub(r"<p:extLst>.*?</p:extLst>", "", new_pres_xml, flags=re.S)

    # Trim presentation.xml.rels: keep every non-slide relationship, plus only the kept slide rels.
    all_rel_entries = re.findall(r"<Relationship\b[^>]*/>", pres_rels)
    new_rel_entries = []
    for entry in all_rel_entries:
        m = re.search(r'Target="slides/(slide\d+\.xml)"', entry)
        if m:
            if f"ppt/slides/{m.group(1)}" in KEPT_SLIDE_FILES:
                new_rel_entries.append(entry)
        else:
            new_rel_entries.append(entry)
    header = re.match(r"(.*?<Relationships[^>]*>)", pres_rels, re.S).group(1)
    new_pres_rels = header + "".join(new_rel_entries) + "</Relationships>"

    # Closed part set: presentation + kept slides/rels + ALL slideLayouts (master declares
    # relationships to every layout, so trimming layouts would leave dangling master rels) +
    # slideMaster + notesMaster + both themes + presProps/viewProps/tableStyles + referenced
    # media (master's own image1/image2 backgrounds + slide23's image3) + root-level parts.
    keep_exact = set()
    keep_exact.add("[Content_Types].xml")
    keep_exact.add("_rels/.rels")
    keep_exact.add("docProps/core.xml")
    keep_exact.add("docProps/app.xml")
    keep_exact.add("docProps/thumbnail.jpeg")
    keep_exact.add("ppt/presentation.xml")
    keep_exact.add("ppt/_rels/presentation.xml.rels")
    keep_exact.add("ppt/presProps.xml")
    keep_exact.add("ppt/viewProps.xml")
    keep_exact.add("ppt/tableStyles.xml")
    keep_exact.add("ppt/theme/theme1.xml")
    keep_exact.add("ppt/theme/theme2.xml")
    keep_exact.add("ppt/slideMasters/slideMaster1.xml")
    keep_exact.add("ppt/slideMasters/_rels/slideMaster1.xml.rels")
    keep_exact.add("ppt/notesMasters/notesMaster1.xml")
    keep_exact.add("ppt/notesMasters/_rels/notesMaster1.xml.rels")
    keep_exact.add("ppt/media/image1.png")
    keep_exact.add("ppt/media/image2.png")
    keep_exact.add("ppt/media/image3.png")
    for n in KEPT_SLIDE_NUMS:
        keep_exact.add(f"ppt/slides/slide{n}.xml")
        keep_exact.add(f"ppt/slides/_rels/slide{n}.xml.rels")
    for i in range(1, 12):
        keep_exact.add(f"ppt/slideLayouts/slideLayout{i}.xml")
        keep_exact.add(f"ppt/slideLayouts/_rels/slideLayout{i}.xml.rels")

    missing = keep_exact - set(names)
    assert not missing, f"expected parts missing from source archive: {missing}"

    # docProps/app.xml: fix descriptive slide-count metadata to match the real trimmed content
    # (7 kept slides, titles taken from the same real deck) rather than leaving stale "62".
    app_xml = read("docProps/app.xml").decode("utf-8")
    kept_titles = ["SemIO", "Was erwartet Euch?", "Warum?", "Informationstechnologien", "PowerPoint-Präsentation", "Mit welchen Informationstechnologien planen Architekten?", "Diagrammnotation"]
    header_titles = ["Arial", "Calibri", "Consolas", "Wingdings", "Office", "Acrobat Document", "SemIO"]
    new_titles = header_titles + kept_titles
    app_xml = re.sub(r"<Slides>\d+</Slides>", f"<Slides>{len(KEPT_SLIDE_NUMS)}</Slides>", app_xml)
    app_xml = re.sub(r'(<vt:lpstr>Folientitel</vt:lpstr></vt:variant><vt:variant><vt:i4>)\d+(</vt:i4>)', rf"\g<1>{len(KEPT_SLIDE_NUMS)}\g<2>", app_xml)
    titles_block_re = re.compile(r"<TitlesOfParts><vt:vector size=\"\d+\" baseType=\"lpstr\">.*?</vt:vector></TitlesOfParts>", re.S)
    new_titles_block = f'<TitlesOfParts><vt:vector size="{len(new_titles)}" baseType="lpstr">' + "".join(f"<vt:lpstr>{t}</vt:lpstr>" for t in new_titles) + "</vt:vector></TitlesOfParts>"
    assert titles_block_re.search(app_xml)
    app_xml = titles_block_re.sub(new_titles_block, app_xml)

    # [Content_Types].xml: keep every Default, filter Overrides to only the kept parts.
    ct_xml = read("[Content_Types].xml").decode("utf-8")
    def keep_override(m):
        part_name = m.group(1)
        return part_name.lstrip("/") in keep_exact
    overrides = re.findall(r'<Override PartName="([^"]+)" ContentType="[^"]+"/>', ct_xml)
    kept_override_entries = [entry for entry in re.findall(r"<Override\b[^>]*/>", ct_xml) if keep_override(re.search(r'PartName="([^"]+)"', entry))]
    # `[^/]+` could never span the `/` in `application/vnd.openxmlformats-...`, so this matched
    # NOTHING and the wave-7 fixture shipped without a single <Default> -- every .rels/.png/.jpeg
    # part left with no resolvable content type, which ECMA-376 Part 2 §10.1.2.2.1 forbids and
    # `decode_pptx` rightly refused. Repaired in wave 14 (see w14-ooxml-cad-parity/).
    defaults_entries = re.findall(r"<Default\b[^>]*/>", ct_xml)
    assert defaults_entries, "no <Default> content types found -- the package would be non-conformant"
    new_ct = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">' + "".join(defaults_entries) + "".join(kept_override_entries) + "</Types>"

    with zipfile.ZipFile(DST, "w", zipfile.ZIP_DEFLATED) as zout:
        for name in names:
            if name not in keep_exact:
                continue
            if name == "ppt/presentation.xml":
                data = new_pres_xml.encode("utf-8")
            elif name == "ppt/_rels/presentation.xml.rels":
                data = new_pres_rels.encode("utf-8")
            elif name == "docProps/app.xml":
                data = app_xml.encode("utf-8")
            elif name == "[Content_Types].xml":
                data = new_ct.encode("utf-8")
            else:
                data = read(name)
            zout.writestr(name, data)

print("wrote", DST)
