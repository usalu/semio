# React

React + reveal.js renderer for `@framework/presentation/core`. 

You MUST use [auto-animate](https://revealjs.com/auto-animate) for declarative transitions.
You MUST NOT animate anything manually and just describe the target slides because reveal.js will auto-animate them.
When many participants morph into a new participant you MUST use ghost target participants because reveals.js can only auto-animate single elements.
When auto-animating many-to-one, then you MUST use ghosts to fix the problem. The source dispositions are morphing into a ghost with the same target position and size. The target disposition already exists in the previous slide as ghost on the target position and size waiting to be faded inplace turning the opactiy from 0 to 100% in by the transition.
A ghost has 100% transparency and cant be interacted (hovered, clicked, selected, etc).