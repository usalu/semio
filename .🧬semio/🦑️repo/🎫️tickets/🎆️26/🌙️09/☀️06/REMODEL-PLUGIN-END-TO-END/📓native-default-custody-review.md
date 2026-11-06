# Native default custody review

NativeDefaultValueCursor output_ready and NativeDefaultSlot ready currently check output presence but not pending recovery. If the value has been produced before an allocation-post or construction-receipt callback interruption, take can move that output before the actual receiving frame is settled. New advance may similarly see the output and bypass the pending effect. Native owner notified; gate on the actual pending recovery and resolve it first.

The slot retains a pending cursor and reports nested loans rather than dropping the owner; the complete typed result and allocation markers require native execution to verify. No tests run.