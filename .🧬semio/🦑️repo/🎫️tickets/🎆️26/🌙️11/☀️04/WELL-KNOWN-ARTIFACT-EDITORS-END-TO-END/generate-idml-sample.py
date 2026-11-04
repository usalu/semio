import sys, zipfile, io
from xml.dom import minidom

PKG = "http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging"
HEAD = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
PARTS = [
    ("designmap.xml", f'{HEAD}<Document xmlns:idPkg="{PKG}" DOMVersion="8.0" Self="d"><idPkg:Spread src="Spreads/Spread_u2.xml"/><idPkg:Story src="Stories/Story_u1.xml"/></Document>'),
    ("Spreads/Spread_u2.xml", f'{HEAD}<idPkg:Spread xmlns:idPkg="{PKG}" DOMVersion="8.0"><Spread Self="u2" PageCount="1"><Page Self="u3" GeometricBounds="0 0 792 612"/><TextFrame Self="u4" ParentStory="u1" ContentType="TextType"/></Spread></idPkg:Spread>'),
    ("Stories/Story_u1.xml", f'{HEAD}<idPkg:Story xmlns:idPkg="{PKG}" DOMVersion="8.0"><Story Self="u1"><ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle/$ID/NormalParagraphStyle"><CharacterStyleRange AppliedCharacterStyle="CharacterStyle/$ID/[No character style]"><Content>Hello IDML</Content><Br/></CharacterStyleRange></ParagraphStyleRange><ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle/$ID/NormalParagraphStyle"><CharacterStyleRange AppliedCharacterStyle="CharacterStyle/$ID/[No character style]"><Content>Gr&#252;&#223;e &amp; Salut</Content></CharacterStyleRange></ParagraphStyleRange></Story></idPkg:Story>'),
]

def build():
    buffer = io.BytesIO()
    with zipfile.ZipFile(buffer, "w") as archive:
        info = zipfile.ZipInfo("mimetype", (1980, 1, 1, 0, 0, 0))
        info.compress_type = zipfile.ZIP_STORED
        archive.writestr(info, "application/vnd.adobe.indesign-idml-package")
        for name, text in PARTS:
            info = zipfile.ZipInfo(name, (1980, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, text.encode("utf-8"))
    return buffer.getvalue()

def validate(data):
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        assert archive.testzip() is None
        names = archive.namelist()
        assert names[0] == "mimetype" and archive.infolist()[0].compress_type == zipfile.ZIP_STORED
        for name in names[1:]:
            minidom.parseString(archive.read(name))

if __name__ == "__main__":
    data = build()
    validate(data)
    assert build() == data
    with open(sys.argv[1], "wb") as out:
        out.write(data)
