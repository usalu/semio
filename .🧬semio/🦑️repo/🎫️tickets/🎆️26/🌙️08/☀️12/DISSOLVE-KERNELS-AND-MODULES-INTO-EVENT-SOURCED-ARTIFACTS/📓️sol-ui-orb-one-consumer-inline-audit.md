# UI Orb One-Consumer Inline Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Component SHA-256: `98f32d1042a8504c8ea73b138ea9df3eb8326af65f3ea84a7042ecab2d033a50`
- Story SHA-256: `a6df96142c0e9057a05b4e3ed713730381566474320b6db16e8d2188ea54ca32`

## Responsibility and Consumers

`🔮️Orb` renders one circular position marker whose normalized `t` value maps to a point on a Ring. Its only independent active production consumer is `⭕️Ring`. The framework React barrel is assembly/glue and the exclusive Storybook story is example/test provenance; neither qualifies as another production consumer.

## Disposition

Inline the Orb implementation and contract privately into the Ring component, delete the standalone Orb component/story identity, and remove its React-barrel and Storybook registry entries. Do not create a shared module or retain a public compatibility export. The implementation is inseparable from Ring semantics and has exactly one production owner.

Execution must wait until the active ClassNames umbrella split finishes because Orb is in that split's direct import closure and its source hash will change.
