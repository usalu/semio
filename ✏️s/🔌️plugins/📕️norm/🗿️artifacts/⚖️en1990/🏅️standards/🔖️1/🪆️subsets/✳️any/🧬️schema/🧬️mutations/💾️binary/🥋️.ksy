meta:
  id: norm_en1990_mutations
  endian: le
seq:
  - id: format
    contents: [0x01]
  - id: tag
    type: u1
  - id: payload
    type: str
    encoding: UTF-8
    size-eos: true
