# Metabolism

While [metabolists](https://en.wikipedia.org/wiki/Metabolism_(architecture)) saw the built environment as an organism that should continuously be able to adapt to the evolving needs of its habitants, I see semio as a metabolistic tool for the evolving needs of a design 🔀

Let me demonstrate metametabolism on metabolism 🤯

## Nakagin Capsule Tower

### Theory ... oh my

There are countless ways to formalize a design ♾️

Assuming I would have been Kisho Kurokawa and used semio, I would chosen the following:

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/examples/metabolism/docs/artifacts-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/examples/metabolism/docs/artifacts.svg">
        <img alt="Capsule Tower Artifacts" src="https://raw.githubusercontent.com/usalu/semio/main/examples/metabolism/docs/artifacts.svg">
    </picture>
</p>

Based on this formalization the design can be synthesized the following:

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/examples/metabolism/docs/computation-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/examples/metabolism/docs/computation.svg">
        <img alt="Capsule Tower Computation" src="https://raw.githubusercontent.com/usalu/semio/main/examples/metabolism/docs/computation.svg">
    </picture>
</p>

### Practice .. yey

A bit overwhelmed? Just take a look at the Grasshopper script `nakagin-capule-tower.gh`💡

#### Requirements
- Grasshopper plugin: Elefront
    > Just use the Rhino Package Manager to install it.
- Rhino model units need to be set to `Meter`.

> Have you already discovered the futuristic version?

When analyzing the folder structure you see that there is a special subfolder `.semio` on the root level. This folder is what turns a normal directory into a kit ⬆️

```
│   nakagin-capsule-tower.gh
│
├───.semio
│       kit.sqlite3
│
├───icons
│       ...
│       capsule_1.3dm
│       capsule_1.svg
│       ...
│
├───prototypes
│       ...
│       capsule.gh
│       ...
│
└───representations
        ...
        capsule_1.3dm
        capsule_1.glb
        capsule_1_1to200_volume.3dm
        capsule_1_1to500_volume.3dm
        ...
```

How the rest of folder is structured is free. As long as the urls resolve to the actual files, you are free choose what ever structure you like. Here the files were grouped after type.

> Did you know that you can change the level of detail? There is a 1to200 version!
