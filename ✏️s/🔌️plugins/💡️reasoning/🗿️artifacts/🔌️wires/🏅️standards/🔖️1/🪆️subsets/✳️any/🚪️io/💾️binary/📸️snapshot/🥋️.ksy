meta:
  id: reasoning_wires_snapshot
  endian: le
seq:
  - id: magic
    contents: [0x89,0x53,0x50,0x4b,0x0d,0x0a,0x1a,0x0a]
  - id: format_major
    type: u2
  - id: format_minor
    type: u2
  - id: required_flags
    type: u4
  - id: optional_flags
    type: u4
  - id: header_crc32
    type: u4
  - id: reserved
    size: 8
  - id: segments
    type: segment
    repeat: until
    repeat-until: _.tag == 0
  - id: footer
    size: 84
types:
  varint:
    seq:
      - id: octets
        type: u1
        repeat: until
        repeat-until: (_ & 0x80) == 0 or octets.size == 10
    instances:
      value:
        value: "(octets[0].as<u8> & 0x7f) | (octets.size > 1 ? ((octets[1].as<u8> & 0x7f) << 7) : 0) | (octets.size > 2 ? ((octets[2].as<u8> & 0x7f) << 14) : 0) | (octets.size > 3 ? ((octets[3].as<u8> & 0x7f) << 21) : 0) | (octets.size > 4 ? ((octets[4].as<u8> & 0x7f) << 28) : 0) | (octets.size > 5 ? ((octets[5].as<u8> & 0x7f) << 35) : 0) | (octets.size > 6 ? ((octets[6].as<u8> & 0x7f) << 42) : 0) | (octets.size > 7 ? ((octets[7].as<u8> & 0x7f) << 49) : 0) | (octets.size > 8 ? ((octets[8].as<u8> & 0x7f) << 56) : 0) | (octets.size > 9 ? ((octets[9].as<u8> & 0x7f) << 63) : 0)"
  segment:
    seq:
      - id: tag
        type: u2
      - id: stored_length
        type: varint
      - id: raw_length
        type: varint
        if: (tag & 0xff00) == 0x0300
      - id: stored_bytes
        size: stored_length.value
      - id: crc32
        type: u4
