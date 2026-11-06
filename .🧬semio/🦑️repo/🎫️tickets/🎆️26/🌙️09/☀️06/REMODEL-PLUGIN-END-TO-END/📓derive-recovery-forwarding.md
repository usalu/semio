# Generated receiver recovery forwarding

Added pending_recovery forwarding from generated record and enum receiver templates to their actual child slots using NativeReceivingEffect borrowed from the value native-decoding contract. NativeReceiveSlot forwarding and the required trait contract are owned by the native agent. Confirm whether optional default receivers and enum selector have their own pending effects before treating traversal as complete. No tests run.

Central signature test migration stopped; unbounded call-site work values removed. Existing tests require real funded work harness and remain uncompiled.
Generated record and enum now implement begin_close_recovered and propagate child slot begin_close errors in normal and close paths. Matcher, selector and NativeDefault close APIs were left as currently declared pending confirmation of their effect custody. No execution.

Contract revised: generated record and enum forward immutable has_pending_recovery and implement gated fallible begin_close directly; no bypass method. Native owner notified. No compile run.
