meta:
  id: wfc2d_mutations
  endian: le
seq:
  - id: magic
    contents: [0x0a, 0x0d, 0x34, 0x7d, 0x3f, 0xf8, 0x53, 0x89]
  - id: format_major
    type: u2
  - id: format_minor
    type: u2
  - id: flags
    type: u4
  - id: domain_tag
    type: u4
  - id: header_crc32
    type: u4
  - id: field_count
    type: u4
  - id: payload_length
    type: u4
  - id: payload
    size: payload_length
  - id: body_crc32
    type: u4
