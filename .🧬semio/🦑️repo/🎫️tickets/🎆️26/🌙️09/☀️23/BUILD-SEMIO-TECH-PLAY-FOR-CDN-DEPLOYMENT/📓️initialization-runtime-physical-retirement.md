# Initialization Runtime Physical Retirement

Native red65056 actually ran1 test,0 passed,1 failed,1218 outside the selector. After releasing the actual Vec backing, a terminal erased cursor Box received23 bytes against its24-byte allocation. Old settlement returned Complete and freed the retained Box, instead of retaining it with Pending0/0. The test cold-drained the runtime before asserting, so failure did not abandon an owned retirement cursor.

The canonical public next_close_byte_demand query now reports the active physical allocation or exact terminal erased Box extent. Displaced-current settlement retains an underfunded terminal cursor, frees it only when its entire extent is admitted, and reports Pending1/exact physical bytes in that same call. Rejected runtime close uses the same settlement implementation; payload child progress is checked against the caller grant. No deferred credit or larger hidden grant is introduced.

A six-case language-neutral fixture shared with Job provides physical Vec extents1,16384,65536 and262144, including insufficient grants. Both settlement and rejected runtime close exercise exact backing release, repeated terminal cursor refusal, retained ownership, funded physical cursor release and complete cold cleanup. Combined current-source native gate4729 is pending. The independent Node/Ajv/JSONPatch Job close-demand source gate already passed1 test across all six neutral cases; this report does not claim full Store or final production acceptance.

Authored source paths: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs and its 🧪️tests/🔬️unit/🦀️.rs. The shared neutral fixture remains unchanged. Both launch registries contain gate11.57.

Actual current native4729 physical runtime case passed0.022s, exercising all12 settlement/close extent rows and terminalBox24-byte denial/full admission DEBUG witnesses. The containing8-case run had an unrelated factory expected subtotal failure (7pass/1fail), so the report claims this selected runtime law only. The later7-case member/durable gate31092 passedall7.
