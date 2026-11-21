# Note canvas events and camera publication

The browser dispatches ink `eventsJson` as a string inside the action arguments. The shell adds the concrete window ID and retains the remaining arguments. Its declaration check qualifies the action, and the native typed producer uses the editor’s declared `command_from_action` bridge. The bridge converts `eventsJson` to `events_json` before decoding the Ink command. The empty argument declaration is for the action surface and does not filter this dispatch.

Browser text and table commits emit `updateBlock` with whole paragraphs or rows, respectively. The Note semantic decoder applies that block to a provisional snapshot. Before the concurrent editing correction, the net-leaf calculation delegated changed blocks to `block_update_mutations`, which emitted only geometry, visibility, lock, name and ink stroke mutations. Thus a text or table content-only update completed as an empty gesture and produced no document owner.

The first transfer probe applied an optimistic camera draft before attempting paste. Its console reported that the camera held the Note input lane for five seconds, and there was no ink publication. A separate transfer-only probe omitted the preceding wheel. It pasted a 12 by 8 SVG, increased the document from zero to one block and exported an archive containing the image. The original paste observation did not establish a clipboard mutation defect.

## Retained camera config receipt

A camera action emits one WindowConfig mutation. After the partition Store commits, the mounted publisher keeps its accepted receipt until it refreshes the exact captured window authority. Successful Store publication advances the partition generation. An actual snapshot transfer installs that newer generation and records the paid receipt for the original caller. The presenter waits for that receipt to settle before reserving the lane result page.

Previously, the next publication unit restarted snapshot capture because the just-completed transfer had emptied its pending snapshot slot. Every subsequent transfer again produced paid progress and deferred the page. This repeated after the generation was already current. The refresh now answers ready when the captured authority has reached the accepted generation and owns no pending snapshot. The existing presenter still waits for its paid receipt to settle and clears the pending Store receipt only after reserving its result page.

The Bun SQLite oracle covers the initial older generation, concurrent custody, the exact accepted generation, and a newer committed generation. Its routing assertion failed before the correction and passed afterward with Rust syntax parsing. The existing Rust mounted law now checks that transfer never recaptures a second snapshot, the accepted generation stays current, and the publisher presents one result before ACK. The native law and browser camera completion need verification on a guest containing the change.
