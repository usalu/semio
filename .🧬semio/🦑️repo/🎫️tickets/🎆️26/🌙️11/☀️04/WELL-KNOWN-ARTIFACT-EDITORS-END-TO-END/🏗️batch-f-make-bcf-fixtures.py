import struct, sys, zipfile, zlib

def png(width, height, rgb):
    def chunk(kind, data):
        body = kind + data
        return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)
    row = b"\x00" + bytes(rgb) * width
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(row * height, 9)) + chunk(b"IEND", b"")

VERSION = '<?xml version="1.0" encoding="UTF-8"?><Version VersionId="2.1"><DetailedVersion>2.1</DetailedVersion></Version>'
MARKUP = '<?xml version="1.0" encoding="UTF-8"?><Markup><Topic Guid="topic-minimal-01" TopicStatus="Open"><Title>Door swing blocks corridor</Title><Priority>Normal</Priority><Labels>architecture</Labels><CreationDate>2026-02-01T08:00:00Z</CreationDate><CreationAuthor>carol@example.com</CreationAuthor><Description>The door leaf collides with the handrail.</Description></Topic><Comment Guid="comment-minimal-01"><Date>2026-02-01T08:30:00Z</Date><Author>carol@example.com</Author><Comment>Please move the handrail.</Comment><Viewpoint Guid="viewpoint-minimal-01"/></Comment><Viewpoints Guid="viewpoint-minimal-01"><Viewpoint>viewpoint-minimal-01.bcfv</Viewpoint><Snapshot>viewpoint-minimal-01.png</Snapshot></Viewpoints></Markup>'
VIEWPOINT = '<?xml version="1.0" encoding="UTF-8"?><VisualizationInfo Guid="viewpoint-minimal-01"><Components><Selection><Component IfcGuid="0HG2A49bzDARlPHy2ZDHwJ"/></Selection><Visibility DefaultVisibility="true"/></Components><PerspectiveCamera><CameraViewPoint X="1" Y="2" Z="3"/><CameraDirection X="0" Y="0" Z="-1"/><CameraUpVector X="0" Y="1" Z="0"/><FieldOfView>60</FieldOfView></PerspectiveCamera></VisualizationInfo>'

def write(path, entries):
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, data in entries:
            info = zipfile.ZipInfo(name, (2026, 2, 1, 8, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, data)

def main(directory):
    parts = [("topic-minimal-01/markup.bcf", MARKUP), ("topic-minimal-01/viewpoint-minimal-01.bcfv", VIEWPOINT), ("topic-minimal-01/viewpoint-minimal-01.png", png(2, 2, (200, 30, 30)))]
    write(f"{directory}/minimal.bcf", [("bcf.version", VERSION)] + parts)
    write(f"{directory}/missing-version.bcf", parts)
    whole = open(f"{directory}/minimal.bcf", "rb").read()
    open(f"{directory}/truncated.bcf", "wb").write(whole[: len(whole) // 2])

main(sys.argv[1])
