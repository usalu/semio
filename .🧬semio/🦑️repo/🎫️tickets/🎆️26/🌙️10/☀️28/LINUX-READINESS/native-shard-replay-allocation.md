# Native Shard Replay Allocation

Actual genuine private Go regression 652197 ended1/null at08:26:34UTC. Replay of2048 same-document events allocated2327832bytes, exceeding the unchanged2MiB regression bound. All12 neutral framing cases passed before the implementation change.

The implementation retains one standard-library JSON decoder per shard, resets its bounded input per scanned line, and checks its input offset plus exact JSON whitespace against each line. Each event remains fresh; unknown fields, sequence, partition, byte/event/document limits, original raw SHA256, file identity and cancellation checks remain. No descriptor retention or platform-specific publication change was made.

Fresh genuine private Go655268 ended0/null at08:27:31UTC, including allocation and all12 framing cases. Full module and actual canonical public route have been started for broader verification. The native120s cold gate is still unaccepted.

Accepted focused allocation is315096bytes for2048events, versus2327832before (86.5percent reduction). Added two further neutral framing rows crossing scanner/decoder refill boundaries, including70000UTF8bytes and70000trailing JSON spaces; their acceptance is pending the queued full module/public route.

The additional unread-whitespace boundary exposed a real offset-accounting defect in the first optimization:666137 ended1/null08:30:44UTC, with all14prior framing cases plus allocation passing. The corrected offset advances only bytes actually fed to the decoder, excluding already-validated unread whitespace discarded on reader reset. The permanent new15th framing vector retains this causal red. New full-module proof started after this correction; earlier655268/658740 and installed ELF predate this final Source change and do not qualify it.

Final full private Go669904 accepted0/null08:31:40UTC after consumed-byte offset correction:35top-level tests and all15 framing vectors, no skips/failures,5.683s Go body. Actual publiccanonical658732 progressed through32projects/150closures and published generation126, then ended1/null08:31:45 at the independently known Rustup foreign-generation guard before tests. This is not a full canonical acceptance. Final native ELF reinstall672947 ended0/null08:32:14UTC; receipt verification is queued. No native cold gate has been rerun against this final ELF yet.

Independent existing public neutral corpus674377 accepted0/null08:34:10UTC, validating current JSON framing with Node JSON.parse, strict closed production IndexEvent through Ajv, Node SHA256, and existing SQLite FTS5 ranking/gzip/fingerprint laws. Verified final installed ELF673749 accepted0/null08:32:59UTC:20386034bytes, SHA2565b38e997aafa808a33a7cd88ce27a9339b2de5ddb5e94e4c35f036bf3fd75dbc, executable0755 and receipt0600.

Preparing a formal5minute authored Source/report/input quiet window after all Root/Course owners ACK. Existing native-layout gate now takes an optional closed lowercase evidence-label argument so new raw proofs do not overwrite earlier generated observations. Its120000ms/16MiB limits, late identity-stage cancellation, exact64shards, physical hashes and generation reuse checks are unchanged. No quiet gate result is claimed yet.

Actual final installed native MCP protocol676165 accepted0/null08:37:05UTC:initialize2025-06-18, tools/list6tools, resources/list8resources, exact reply IDs1/2/3, no errors/forced cancellation,27ms native body,6998stdoutbytes. This protocol smoke check does not execute listed tools or prove native search. Independent neutral674377 totals8passed66assertions.

## Formal Quiet Gate

All8Root/Course owners ACKed the final08:41:00–08:46:00UTC authored Source/report/input quiet interval before its start. Root launched the normal-funded gate at08:41:07; capacity admitted it later, with native work beginning approximately08:42:15. No intentional authored writes occurred before explicit release08:46:26. This is coordinated operational evidence, not an independent atomic whole-Source hash proof.

Actual gate678847 ended1/null08:45:10UTC. Late identity-stage cancellation succeeded at54.449s, settled38ms after SIGINT, produced0stdout and preserved the prior physical snapshot. Cold search then hit unchanged120000ms with forced cancellation,0stdout and no warm/completion retry. Its snapshot became ready at69.470s; second Source fingerprint began71.246s and ended114.564s, after which collection/indexing reached201279before the timer. All64physical shards and warm generation reuse remain unaccepted. Decoder memory improvement does not prove this overall gate. Generated exact stage receipt:native-decoder-formal-stages.json. Next investigation is why the post-snapshot fingerprint costs43seconds, versus16.685seconds before snapshot construction.

## Recovered Diagnostic

Normal bounded container observer731926 ended0/null08:59:47UTC and recovered actual completed private ticket generated artifacts:9090byte outcome and33083byte stderr. The host redirected stream log was lost; recovered progress timing is only a coarse reference to preceding runtime GC starts. See generated/native-post-snapshot-recovered-profile.json. The16MiB limit is output capture, not a measured Go heap limit; no16MiB-heap-thrash claim is supported. Diagnostic remains failed120s with no acceptance or completion retry.
