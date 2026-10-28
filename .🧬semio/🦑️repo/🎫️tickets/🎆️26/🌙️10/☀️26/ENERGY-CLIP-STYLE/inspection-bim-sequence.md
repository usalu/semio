# Inspection BIM Consuming Sequence

RED reproduced the exchange mapping family error using the actual authored _seq helper with a native Square FadeIn followed by ReplacementTransform to two Circles. There are no aliases or updater callbacks in this control; it fails the same strict native family zip (19.693 seconds). The outer AnimationGroup(lag_ratio=1) initializes both stages eagerly, so the later morph aligns/mutates the source family after the first fade captured its initial family.

The native Succession implementation initializes/setup/begins only its active stage and advances at the boundary. The renderer agent changed only the outer _seq container to Succession, preserving parallel tuple stages, geometry and identity. The language-neutral fixture samples five timeline positions; GREEN requires native source consumption and exact Cairo equality with the independently rendered two-Circle endpoint. Focused GREEN and current bilingual exchange validation are pending.
