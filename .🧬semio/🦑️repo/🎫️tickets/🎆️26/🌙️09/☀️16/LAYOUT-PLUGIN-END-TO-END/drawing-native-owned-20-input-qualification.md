# Native20 Input Metadata Qualification

A preparation mistake before Native21 launch overwrote historical Native20 input JSON. The copied Python collector retained a dynamically assembled `-20-inputs.json` output stem. The replacement bytes are a later504-input preparation snapshot, SHA256 0948f2c1b215ebe20a6a2ad63dc8be54ea36d43a1bb7abc4ceeb092f3b47c14e. They are not the historical Native20 authority. Expected historical SHA256 from the unchanged Native20 command is36c4be89b62d1c59af59ad099d21547962068c2f797c75d1e9ea1dc393e54e87.

One bounded read-only recovery check ran through `repo tools exec --light git -- show HEAD:<exact packet path>`. Git returned128: the path is not present in HEAD. No guessed reconstruction, source restoration, Git mutation or extra recovery branch followed.

Native20 actual failed owning command, logs and result remain unchanged. Its recorded runtime partial passes and tuple failure are genuine; the historical complete input roster can no longer be independently recovered from that path, so its source-capture evidence is qualified. Native20 was not an accepted complete owner proof. The later overwritten bytes must not be used as a Native20 replay authority.

Native21 uses a corrected fresh immutable output stem, refuses to overwrite an existing packet, and its runner verifies the declared complete packet hash before any source precheck or semantic launch. Native21 has not launched at this audit point.
