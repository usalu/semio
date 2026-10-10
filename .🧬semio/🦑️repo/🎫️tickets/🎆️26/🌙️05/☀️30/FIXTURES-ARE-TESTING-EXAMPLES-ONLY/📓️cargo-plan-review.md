# Cargo Physical Plan Review

Read-only Source cut in 📥️cargo-plan-review.json. No executed test or Cargo claim.

The same discoverPlan/membersPlan/selectPlan now feeds sync drain and controlled drain. Original matcher creation remains outside loops, document caches remain per membership invocation, and nearest workspace selection retains explicit isolated tests. Controlled checks original signal before every yielded operation, after physical execution and after awaited advance; generator.return in finally closes the suspended plan on operation/advance/cancellation errors. Counts increment only after successful physical operations and match the genuine Progress definition (state/presence/list/text, path, positive completedOperations). The initial plan.next occurs outside try but has no held resource in current generators.

Concrete gaps before claiming current controlled physical custody:

- operate(text) calls synchronous physical() internally before readFileSync. Those additional ancestry lstat operations bypass reported per-operation progress/cancellation. PhysicalPlan already emits ancestry states, so the final guarded text port must intentionally handle its immediate guard without pretending every physical operation is represented, or emit/control the guard operations through the plan. The current three-operation cancellation law does not cover this hidden state work.
- operate(list) directly readdirSync(path) without fresh ancestry validation. A continuation after a parent listing can replace a queued child with a symlink; the resumed walk lists the linked directory before manifest physical validation catches it. Root's planned final-per-operation physical guards are required before list/presence, not only text.
- physicalPlan stores lstat results across awaited advance and checks those stale results on resume. A replacement at the final leaf continuation can change the actual file identity. Text rechecks symlink ancestry synchronously, but does not compare dev/ino or original bytes; a regular replacement is admitted. Completion after final advance similarly has no current-state fence. Add actual symlink/regular replacement and final-continuation mutation laws before defining a current-source result contract.

The current API returns an observed roster, not a promised stable snapshot. Its DTO has no observation hashes/currentness. Selected preparation must therefore preserve its existing separate current-source/custody/roster guards across await and before publication/child acquisition. Do not infer whole preparation coherence merely from sync/controlled equality on a static tree.

Tests should retain main matcher-count and parse-once assertions, add mid-membership cancellation, pre-aborted zero operations, queued child symlink replacement and final-leaf identity change. Independent fast-glob/TOML gives membership semantics, while original lstat/read witnesses give physical custody. No new corpus schema required.
