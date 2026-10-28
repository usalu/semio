# Recursive Value Test Obligation Supersession

The existing details test `type_select_input.test.ts` intentionally replaces `SELECT refuses oversized or mixed unsupported domains without truncating members` with `SELECT recursively pages actual large domains without inventing required values`.

The independently closed Measure86 reference revision has SHA-256 `8bafe9b88f0141abc138674132dd5fb4ee60fb1c5330aff8560e9d1946455fde`. The frozen Value TS6 revision has SHA-256 `089ce44f5f3830efa2acadac1832d64e775d30231d727bd3db79269ec42c7de3`.

The superseded obligation was a frontier refusal for more than32 SELECT leaves. The replacement checks genuine31-choice pages, exact first/last leaf names, the next31-choice page, both still-missing required fields, continued central command refusal, and unchanged source/caller payload. It raises no choice capacity and invents no required value. The original failing assertion remains in `🗑️generated/type-value-select-consumers.txt`; the corrected narrow law passes1test/11assertions and the full prior SELECT file passes15tests/4954assertions. A new whole114-test regression run is not claimed.

The new recursive Value suite separately passes37tests/9850assertions. Its later immutable test-request repair was replayed as1test/11assertions; the producer was unchanged. Native execution and publication remain false.
