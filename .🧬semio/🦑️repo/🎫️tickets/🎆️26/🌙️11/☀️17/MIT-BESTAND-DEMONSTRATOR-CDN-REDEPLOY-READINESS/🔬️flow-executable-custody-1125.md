# Flow staged executable custody

The staged Flow release receipt binds `semio-hub-flow-component` to managed run `wNiwDG4`. Its compiler observation started November 24 at 15:46:33 UTC. The receipt and per-component leaf roster agree on 1,044 observed units, source tree digest `4ce2a61e84973c671c6077dc1cf5d53bc70a65ab68ac16d801bb07be33816d46` and provenance root `14c862ac5700730e1af1e61bb52f5f2129467a63d591c8d6f2a3b62d121f8cab`. The receipt names the original 275,307,971-byte WASI component at SHA-256 `ce4a1fe1a1619471b1624faded79b379a3e0745399c574cace455ce71e6d40ce`.

I expanded the observation's indexed source paths, matched the exact inputs of `semio-os-flow-host-read` and `semio-os-flow-host-session`, and hashed the current source files independently.

| Consumed file | SHA-256 in Flow compilation | SHA-256 now |
| --- | --- | --- |
| `📥️evaluation-source/🔎️read/🦀️.rs` | `a581da059450eeb8218c938fd7376ffb7b43c42d0d79eb3fcce31a75d725ae3d` | `0cdd59eed6e4976c475ffd1bc4d3cb5f01c584ac0edbb013d2d777d7be3817b4` |
| `📥️evaluation-source/🔎️read/✏️authoring/🦀️.rs` | `67bc72de7144b4ec85d3b4c240cfdc92246d357ce911714f9a4815bdf0da59de` | `e152d2234ef5099fb16885347038aeafc9a6461ae519dee2d3ac5578be096010` |
| `🧾️eval-session-state/🦀️.rs` | `c51fd6bddd338ead62cc11651cc34d792e060d386f634830f4572c21a08051f5` | `709d7447dd7be700670aa05cae9ec41b7ed0d4e64e2670d2262eaa5a97bb5340` |

The first two sources own the `settle_close` and `close_with_poll` caller imports and the third owns the repaired cancellation admission quote. The 193,858,034-byte staged browser core is `9618d0e210ad5697ff43f643f233030e839f82ac4fb80b89d8a3cef945c2a536`. Independent gzip inflation of the 48,520,681-byte current Vite core returned the same SHA-256. The authored descriptor comparison agrees semantically, but proves no executable freshness for these changed Flow operations. Flow is therefore included in the remaining native receipt queue. No standalone Flow browser acceptance proof was produced during this custody inspection.
