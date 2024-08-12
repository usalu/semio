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

### Practice ... yey

A bit overwhelmed? Just take a look at the Grasshopper script `nakagin-capule-tower.gh`💡

#### Requirements
- Rhino 8
- Rhino model units need to be set to `Meter`.

When analyzing the folder structure you see that there is a special file `.semio/kit.sqlite3` on the root level. This file is what turns a normal directory into a `kit` ⬆️

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

#### Speckle, ... what? I thought semio is about files 📂

Looking closely at the `types`, you see that some `representation` are not linking to local files but instead they link to speckle models. You are free to use whatever storage, collaboration and version environment you like (Speckle, Git, Nextcloud, Spline, ...). As long as the `url` (Unique Resource Locator) leads to a resource and you know how to fetch it, you can use all the advantages of semio 🕸️