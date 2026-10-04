# Ticket Output Space Recovery

Native BMP green1, BCF5 and XLSX4 failed after the shared volume ran out of space. Read-only inspection confirmed the following ticket-owned generated directories had no live process references; the bounded worker additionally confirmed its three old targets are inactive. Source inputs, reports, logs, active targets, shared Cargo cache and other chats remained outside this cleanup. The general clean skill was not executed because its process-killing and cross-ticket deletion conflict with the user’s concurrent-work instructions. Initial attempts could not write a report or a shell here-document because of ENOSPC and deleted nothing. A direct Python command removed only the verified obsolete full-catalog component output; a subsequent pass removed the remaining verified inactive directories.

| Generated directory | Apparent bytes | Files |
| --- | ---: | ---: |
| full-catalog-component | 1563170600 | 1194 |
| nx-data | 236437708 | 1160 |
| nx-data-retained-clone | 230013905 | 1037 |
| nx-data-retained-clone-native-5 | 230020329 | 1020 |
| nx-data-retained-clone-source-6 | 230016757 | 1020 |
| nx-data-retained-clone-source-7 | 230028624 | 1020 |
| nx-data-retained-clone-source-8 | 21108588 | 2 |
| nx-data-retained-clone-source-9 | 230067022 | 1020 |
| nx-data-retained-clone-native-6 | 230072104 | 1020 |
| retained-opc-target | 620292870 | 1247 |
| retained-cursor-handoff-target | 394784269 | 1135 |
| docx-preflight-target | 619515553 | 1260 |

Removed 12 verified inactive directories; free bytes after deletion: 5028737024. Fresh native reruns are required; failed ENOSPC runs are not test passes.
