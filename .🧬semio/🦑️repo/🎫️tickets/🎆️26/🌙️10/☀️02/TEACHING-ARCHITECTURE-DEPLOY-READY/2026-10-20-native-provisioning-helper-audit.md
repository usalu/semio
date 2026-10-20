# Native Provisioning Helper Review

Read-only candidate review; no helper/test command ran. Initial script1b11d0da…/17314B and updated63702c4104d45065db83884cdaa062b96b17f2b68067d8308aa6ba127659d832/17603B were inspected with neutral tests/schema/cases.

## Corrected Findings

Initial closed pin admitted only failed n/n2 paths; Runtime added Root-selected fresh n3 in updated draft. Initial guest failure cleanup unlinked/rmdir based only on created boolean, without inode custody, risking deletion of replaced unowned entries. Updated draft removes deletion entirely, closes only its FD and honestly retains failed scratch. Initial postinspect was skipped when action/cancellation threw; updated run now attempts bounded10second postidentity observation after action failure. These corrections were inspected, not executed.

## Remaining Actionable Issue

Updated final probe run waits inspect(true), which deliberately ignores cancellation/deadline for required postidentity grace. If cancellation arrives during that final inspection, the successful result returns without ordinary check() and provision returns success; main can write accepted:true alongside cancelled:true. Reinstate ordinary check after mandatory postinspect and before success; add cancel-during-final-postinspect and deadline-during-final-postinspect neutral cases, ensuring postidentity still settles before refusal. Sent directly to Runtime.

## Evidence And Test Limits

Host stableRead has nofollow single-link/descriptor/name/time/size fences and closes in finally; ELF rejects32bit/wrong endian/machine,invalid type/truncated/out-of-bounds tables and interpreter. Pin binds real digest/bytes/current helper+schema and fixed container/source identity. Provision first authenticates projected exact ID/image/token/mount/running state; transfers only pinned binary bytes via stdin to fresh /tmp directory/exclusive file, then exact absolute --help, no product Source transfer. Guest environment removes only SEMIO_SCRIPT_PROCESS_INVOCATION; destination PATH retained. No whole-builder stop, receipt fabrication or old bun-repo command exists. Host transport failure explicitly does not prove guest terminal/cleanup; bounded guest timeout remains separate.

Prepared tests consume actual helper functions and exact generated guest program through controlled syscall ports. Ajv independently validates pin; Python struct/hashlib oracle checks ELF/hash. They are meaningful neutral tests but unexecuted. Missing focused coverage: actual host stableRead symlink/hardlink/descriptor/name/race/overlimit/fd-close vectors; host pin/receipt collision before any port action; stopped builder through provision proving zero receive/probe rather than only standalone identity check; final postinspect cancellation. Initial transport schema cases test wrong identity before receive and only fixed semantic actions, not actual Docker argv transport. No claim of completed neutral suite or actual guest installation is justified.

Narrow publisher fix independently reviewed separately at native-go-publication-defect-audit.md; actual fresh n3 publication remains Root proof before pin/install. Current helper is not ready for actual install until remaining cancellation issue is resolved and neutral tests execute.

Latest script independently matches3dc827e66bf8de7f1ec66109dba59668eef09bd9602125f205f12e8d74a0f2ba/17611B and now calls ordinary check() immediately before successful provision return. This closes the identified final-postinspect cancellation/deadline success gap at Source level. Release reports expanded56neutral rows prepared; this auditor has not executed or fully re-reviewed that latest test corpus, so no neutral pass is claimed.

## Frozen56Row Neutral Corpus Independent Review

Actual frozen identities verified: script3dc827e6…/17611B,tests e7dc278ba54627565e727fe6b78dc7119869fc51e768387d68b80f51ec6f2689/13046B,schema66336db2…,casesf7bce648…. Corpus has8pin+8ELF+8identity+14transport+5real-file+13generated-guest=56rows, all consumed by corresponding loops. Neutral entry imports helper; import.meta.main guard prevents main/pin/install execution. It imports no dockerPort and invokes only provision with fake semantic port; exact generated guest program receives fake fs/process/child_process modules. No real Docker transport,actual retained identity read,host binary copy,guest action or external Source mutation is reachable through these test calls. Real host filesystem writes are unique generated/nf-* scratch with finally cleanup; real Python subprocess only evaluates supplied neutral JSON/hex in memory and SQLite in-memory.

Ajv independently validates closed fixture corpus and own pin decisions. Python struct parses ELF fields/tables and hashlib compares actual neutral byte digest; SQLite canonicalized json(json_extract(events)) comparison against json_array removes insignificant whitespace without changing transcript semantics. SQL also checks early-no-receive for stopped/unpinned/cancel/deadline/wrongidentity rows. This is meaningful independent representation/trace checking, not a second actual transport.

Final cancellation/deadline rows mutate controls during fourth inspection and require6calls/4inspections/1receive then refusal, exercising the corrected final check. Stopped-before and unpinned-binary refuse before receive. Guest collision asserts zeroopen/write/spawn; guest name/descriptor/symlink/hardlink/readerror rows test actual generated stableRead and balanced descriptorclose. Five real host cases cover valid,empty,overlimit,directory,hardlink; host pin/receipt preflight collision and true concurrent descriptor races are still not exercised. No actual Docker argv/cancel-disconnect behavior or native binary usability follows from neutral corpus.

No concrete static blocker found in frozen neutral entry. Root owns actual running96264; this read-only review does not claim that lane collected or passed. No frozen input edited.
