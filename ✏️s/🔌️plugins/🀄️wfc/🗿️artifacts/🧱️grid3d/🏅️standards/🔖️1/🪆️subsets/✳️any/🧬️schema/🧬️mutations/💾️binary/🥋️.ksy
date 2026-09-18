# 🥋 wfc.grid3d.mutations — derived from 📡️.protocol.semio; one Kaitai attribute per declared field.
meta:
  id: mutations_grid3d
  endian: le
seq:
  - id: header_format_major
    type: u2
  - id: header_format_minor
    type: u2
  - id: header_flags
    type: u4
  - id: header_domain_tag
    type: u4
  - id: header_header_crc32
    type: u4
  - id: body_payload
    size-eos: false
    size: 0
  - id: footer_artifact_mark
    type: str
  - id: footer_body_crc32
    type: u4
