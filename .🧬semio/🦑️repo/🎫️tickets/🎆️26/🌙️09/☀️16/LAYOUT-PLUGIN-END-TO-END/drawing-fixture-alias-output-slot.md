# Fixture Alias Output Slot

The real OriginalJobFixtureCaller alias helper now retains the token in its own outer slot while invoking the canonical StepContext owned-output helper. It no longer adds an unconditional fuel unit after success: the defining actual alias work is already folded by StepContext. Existing node identity and exact physical receipt assertions remain. This is a test-only caller migration; genuine post-refusal custody laws remain in the defining Root StepContext test slice. No runtime was performed.
