# Selected Native Descriptor Census — 2026-10-29

Both normal course jobs stopped before their targets: semantic 13235 exited 1 with filesystem selection status 1, 108 stdout bytes, empty stderr, complete NUL/newline tail and one own guard (expected PID 659329); parent-owned preview 64240 subsequently exited 1 at the same boundary with expected PID 658503. Neither result establishes TSC execution, frontend readiness or an open product port.

## Causal reproduction

The retained input descriptor-census-probe.ts runs through the real test-issued Original, issued resource control, owned command capture and artifact descriptor helper. It creates only a private generated directory and holds one real readonly directory descriptor. Its before/after handle stat and named lstat agree on device and inode.

With that descriptor held, empty recursive selection and exact selection return status 0. Writing a closed child file changes recursive selection to status 1, with empty stderr and byte-identical positive descriptor output. The PID, descriptor number, path and inode remain unchanged. This reproduces a command-search exit rather than evidence of a disappearing owned PID or an early descriptor close. Initial probe 19481 intentionally failed its all-success expectation; refined semantic probe 84463 exited 0.

The actual command is lsof -nP -F0pcfatn +D ROOT. Upstream explains that +D expands descendants into individual search terms; its default status reflects whether all requested terms matched. A closed file can therefore cause status 1 even when the held root descriptor is correctly reported. The documented -Q option changes unmatched search terms to successful status without suppressing generic non-search failures. Sources: [upstream tutorial](https://github.com/lsof-org/lsof/blob/master/docs/tutorial.md), [upstream FAQ](https://github.com/lsof-org/lsof/blob/master/docs/faq.md), [4.95.0 manual](https://github.com/lsof-org/lsof/blob/4.95.0/Lsof.8).

## Narrow correction

The existing retirement contract now selects -Q +D ROOT only for its recursive fallback. Mounted-volume selection stays +f -- ROOT; exact file queries and the Windows alternative remain unchanged. No process-ID selector, output truncation, retries, guard limits, policy budgets or descriptor acceptance condition changed.

The complete inventory assertion still requires status 0, no signal, empty stderr, bounded complete NUL records, the exact own PID, every declared readonly guard and no foreign or undeclared descriptor. A search that returns no positive guard remains a refusal.

The actual observed tool is lsof 4.99.7. The option is also documented in upstream 4.95.0. The devcontainer declares Ubuntu 24.04 and installs distribution lsof without a repository version pin. This test does not claim execution on native macOS or Windows. Their existing mounted/alternative branches receive no new argument; an unsupported recursive-tool option remains a fatal nonzero/stderr refusal, with no compatibility bypass.

## Test-driven evidence

New portable JSON schema and two-case corpus specify mounted and recursive selector arguments. The permanent regression extracts the actual Source selector expression through TypeScript's AST; it does not substitute a copied selector implementation. The language-neutral corpus validates with Ajv. Actual lsof provides the independent process/descriptor oracle.

Before the production edit, permanent baseline 5824 exited 1: corpus expected -Q and the populated actual query still returned 1. Final focused run 41579 exited 0: 4 tests, 47 assertions, 0 failures in 2.90 seconds. It covers real empty/populated trees, unchanged positive wire/inodes, missing guard despite status 0, invalid-option generic status 1 with stderr, unexpected own descriptor, genuine owned foreign child, wrong PID, truncated wire, existing bystander retry and cancellation.

An additional attempt to run the older broad cache fixture directly through the lightweight Bun route exited before its body because that file requires an inherited Original and does not declare its own test envelope. It executed no descriptor law and is not claimed as a pass. Its guard was preserved; the new dedicated regression declares and receives the genuine test envelope correctly.

Logs: generated descriptor-selector-baseline.log, descriptor-selector-fixed.log, descriptor-census-search-semantics.log and descriptor-custody-regression.log. All live focused handles are terminal; there is no new global graph job or automatic semantic retry.

## Files

- Retirement production selector: framework process artifacts native-build/storage/cleanup/retirement/contract/TypeScript owner.
- Permanent regression: cache/descriptors/tests/selection/TypeScript owner.
- Portable corpus and schema: cache/descriptors/tests/JSON and schema.json.
- Retained input: descriptor-census-probe.ts.
- Completion ledger: completion-gates-2026-10-28.json and Markdown addendum.

Full semantic, unit, watcher, complete browser topologies and persistent product readiness remain required. The focused result establishes this causal selector correction only.

## Platform follow-up: cross-platform readiness is not yet complete

The initial unchanged-mounted-branch observation does not establish all Darwin cases. The directory-partition backend is explicitly declared for Darwin, Linux and Windows in storage/backends/schema; selecting it on Darwin reaches the recursive branch. Apple publishes system lsof4.91 without -Q, while main.c still sets failure for each unmatched Sfile. Therefore the Linux-tested correction needs an owned modern tool/provisioning route for Darwin directory storage. It is not yet a complete cross-platform repair. [Apple option and exit implementation](https://github.com/apple-oss-distributions/lsof/blob/main/lsof/main.c), [Apple4.91 manual](https://github.com/apple-oss-distributions/lsof/blob/main/lsof/lsof.8).

No lsof entry or provisioner exists in the current process tool registry; bare lsof passes through its existing command funnel. The reusable native-tool provisioner already verifies archive size/SHA, bounded extraction, executable receipts and atomic publication through caller ports, but its Native-backed tool store cannot bootstrap the census tool by recursively requiring that same census during retirement. The correct pre-Native provision boundary is under investigation. No unsupported-option fallback, nonzero acceptance, manual installation or new Source edit has been added. Parent is separately authorized to seek actual Linux runtime evidence using the proven installed4.99.7; that does not satisfy Darwin readiness.
