# React

React + reveal.js renderer for `@framework/presentation/core`. 

- You MUST use [auto-animate](https://revealjs.com/auto-animate) for declarative transitions.
    - You MUST NOT animate anything manually and just create the right elements with matching data ids because reveal.js will auto-animate them.
    - reveal.js only supports one-to-one morphing and doesnt support many-to-one or one-to-many auto-animations, hence you need to "hack" with ghosts.
        - When morphing many-to-one, then you MUST use target ghosts to fix the problem. The source dispositions are morphing into a target ghost with the same target position and size and during the morph the transparency is increased from 0 to 100% when on target position. The target disposition is already on the final position and size and has the reverse transparency from 100% at the start of the morph to 0% transparency at the end of the morph.
        - When morphing one-to-many, then you MUST use source ghosts to fix the problem. The target dispositions already exist one slide earlier as source ghosts. When the morph starts the one is immediately turned from fully 100% opaque to fully 100% transparent and the source ghosts are turned immediately from 100% transparent to 100% opaque. During the morph only the the position, size and style updates.
- You MUST NOT distort any element and you MUST always cover as much without leaving any white space.
    - You MUST use `object-fit: cover` only.
        - You MUST NOT use `object-fit: fill`, `object-fit: contain`, `object-fit: scale-down`