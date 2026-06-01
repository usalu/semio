# React

React + reveal.js renderer for `@framework/presentation/core`. 

You MUST use [auto-animate](https://revealjs.com/auto-animate) for declarative transitions.
You MUST NOT animate anything manually and just describe the target slides because reveal.js will auto-animate them.
When many participants morph into a new participant you MUST use ghost target participants because reveals.js can only auto-animate single elements.