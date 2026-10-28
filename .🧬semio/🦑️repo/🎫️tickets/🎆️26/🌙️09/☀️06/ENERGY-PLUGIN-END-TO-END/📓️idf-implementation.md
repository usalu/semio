# IDF Interchange

Official positional definitions come from EnergyPlus v25.2.0 `idd/Energy+.idd.in`, processed by the matching upstream `idd_parser.py` and `modify_schema.py`. The retained dictionary covers the model codec and every object in upstream `testfiles/1ZoneUncontrolled.idf`.

Official dictionary SHA-256: `2aae53d57fefeab6529221fb8e87b80a41135f09e727e1bbae53be4367f72911`.

Fixture source: https://github.com/NREL/EnergyPlus/blob/v25.2.0/testfiles/1ZoneUncontrolled.idf

Implementation delegates physical model conversion to existing epJSON IO and preserves its explicit unsupported diagnostics. IDF lexer retains positional blanks, accepts comments, BOM, CRLF and Unicode, and refuses truncated records and unrepresentable delimiter characters.

## Validation

The production Rust lexical module compiled with the repository-pinned nightly Rust. The ticket `🧪️verify-idf-eppy.py` verified all 88 official fixture objects against Eppy 0.5.69 plus all five language-neutral lexical vectors. Command: generated Python runner with this ticket script. Result: PASS. Full import/export crate tests are being coordinated separately.

## Official Converter Conformance

All four committed BESTEST epJSON documents were exported through the production IDF codec, converted back by official EnergyPlus 25.2.0, validated against the official schema, and compared structurally with their original epJSON. Every named object and field matched. Synthetic epJSON dictionary keys of IDF objects without a Name field are intentionally ignored because IDF has no position that could carry those keys. The production codec also converted the upstream 88-object IDF fixture to epJSON exactly equal to the official converter. Four production pure-codec Rust tests pass.
