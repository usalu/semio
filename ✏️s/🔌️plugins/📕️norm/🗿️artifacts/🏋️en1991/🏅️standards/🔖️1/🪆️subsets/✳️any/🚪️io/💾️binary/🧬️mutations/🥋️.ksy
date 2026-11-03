meta:
  id: norm_en1991_mutations
  endian: le
seq:
  - id: format
    contents: [0x01]
  - id: tag
    type: vlq_base128_le
  - id: len
    type: vlq_base128_le
  - id: payload
    type: str
    encoding: UTF-8
    size: len.value
