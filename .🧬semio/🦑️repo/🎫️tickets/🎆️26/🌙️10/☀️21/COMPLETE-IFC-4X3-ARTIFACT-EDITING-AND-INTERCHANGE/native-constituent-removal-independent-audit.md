# Constituent Argument Removal Source Audit

Read-only narrow helper/routing review2026-10-21. No new confirmed P1/P2. Native uncompiled/unexecuted.

constituent_argument_collection now accepts Insert/Remove at exact6field complex/part/args/index. Insert admits numeric<=len and dash=end;Remove requires numeric<len and never dash. Returned prefix still ends at args tuple. Five routing uses consistently preserve part/argument,pull actual parts args into aggregate,run tuple-aware value editor,and restore resulting args before SetEntityComplex.

remove_argument shortcut tests path.length==prefix;for whole constituent argument removal path extends beyond args-prefix,so it cannot accidentally bypass tuple editor or emit primary RemoveEntityArg. Existing DomainWork validates changed final arity/domain and preserves unchanged-invalid/nonworsening policy;optional Unset is distinct from removing declared slot. No new owner/close/demand branch;previous retained tuple cancellation custody applies.

Worker9neutral actual TS/inverse+independentJSONpatch and strictpass are reported,not independently rerun. Native first/middle/last surplus removal,optional-vs-Unset,changed-invalid and everyprefix laws remain authored only. This condition review does not establish runtime accounting or full schema/nativeStore proof.
