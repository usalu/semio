<p align="center">
    <a href="https://github.com/usalu/semio">
        <picture>
            <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/refs/heads/main/assets/logo/kit_horizontal_dark.svg">
            <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/refs/heads/main/assets/logo/kit_horizontal.svg">
            <img alt="semio kit" href="https://github.com/usalu/semio/" src="https://raw.githubusercontent.com/usalu/semio/refs/heads/main/assets/logo/kit_horizontal.svg">
        </picture>
    </a>
    <br/>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/designs.svg" alt="Designs"></a>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/pieces.svg" alt="Pieces"></a>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/connections.svg" alt="Connections"></a>
    <br/>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/types.svg" alt="Types"></a>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/ports.svg" alt="Ports"></a>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/attributes.svg" alt="Attributes"></a>
    <br/>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/models.svg" alt="Models"></a>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/lods.svg" alt="LoDs"></a>
    <a><img src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/badges/tags.svg" alt="Tags"></a>
    <br/>
    <i>🫀 Metabolism</i>
</p>
<br/>

While [metabolists](<https://en.wikipedia.org/wiki/Metabolism_(architecture)>) saw the built environment as an organism that should continuously be able to adapt to the evolving needs of its habitants, I see semio as a metabolistic tool for the evolving needs of a design 🔀

Let me demonstrate metametabolism on metabolism 🤯

![Design Variants](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/design-variants.png)

# 🗼 Nakagin Capsule Tower

## 📚 Theory ... oh my

There are countless ways to formalize a design ♾️

Assuming I would have been Kisho Kurokawa and used semio I would chosen the following:

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/artifacts-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/artifacts.svg">
        <img alt="Capsule Tower Artifacts" src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/artifacts.svg">
    </picture>
</p>

Based on this formalization the design can be synthesized like this:

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/computation-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/computation.svg">
        <img alt="Capsule Tower Computation" src="https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/computation.svg">
    </picture>
</p>

## 🔨 Practice ... yey

A bit overwhelmed? Just take a look at the source files 💡

### ✏️ sketchpad

![Sketchpad Overview](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/sketchpad-demo.gif)

### 🦗 Grasshopper

> Make sure to open Rhino 8 and set the model units to `Meter` Ⓜ️

Just open the main Grasshopper definition `nakagin-capule-tower.gh`, hit the run toggles and enjoy 🍸

![Grasshopper Parametric Design](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/grasshopper-definition.png)

The `Capsule Dream` design should take around a minute to load the geometry 🐚

![Design Variants](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/design-variants-rhino.png)

Every model has a 1to500 model:

![Rhino Default Variant 1to500 volume](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/rhino-default-variant-1to500-volume.png)

and a 1to200 model:

![Rhino Default Variant 1to200 volume](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/rhino-default-variant-1to200-volume.png)

Changing the typology is a breeze 🔁

![Rhino Futuristic Variant 1to500 volume](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/rhino-futuristic-variant-1to500-volume.png)

1to200 is still there 🪄

![Rhino Futuristic Variant 1to200 volume](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/rhino-futuristic-variant-1to200-volume.png)

In a conventional block-based approach, block instances have insertion planes and are thus not linked 🪨

Switching from one to another variant would look either like this:

![Block-based Default to Futuristic](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/block-based-default-to-futuristic.png)

or this:

![Block-based Futuristic to Default](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/block-based-futuristic-to-default.png)

Due to the port-mechanism of semio, types are not inserted but instead linked 🔗

Further they have metadata attached:

![Block-based Futuristic to Default](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/rhino-default-variant-metrics.png)

### 💻 Integrations

Looking closely at the `types`, you see that `model` are not (only) linking to files but instead they link `urls`. This enables you to use any storage, collaboration and version environment you like (Speckle, Dropbox, OneDrive, GitHub, Nextcloud, Spline, ...) or all of them together 🕸️

[Here](https://app.speckle.systems/projects/e7de1a2f8f) you can view the models even on your phone 📱

![Speckle Models Overview](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/speckle-models-overview.png)

Or load the geometry directly from Speckle into Grasshopper ⬇️

![Speckle Models Overview](https://raw.githubusercontent.com/usalu/metabolism/refs/heads/main/docs/grasshopper-speckle.png)

> The Speckle Grasshopper plugin is not particulary the best 🥴 Most likely your receive components are not fetching and instead showing errors or show no errors and still not fetch ❌ If you replace them with fresh receive components and hit `Receive` then it should work ✅ And yes, you have to do that every time you open the file again 🥵

### 🤔 Curious, how it works?

When analyzing the folder structure you see that there is a special file `.semio/kit.sqlite3` on the root level. This file is what turns a normal directory into a `kit`⬆️

```
│   nakagin-capsule-tower.gh
│   ...
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
└───models
        ...
        capsule_1.3dm
        capsule_1.glb
        capsule_1_1to200_volume.3dm
        capsule_1_1to500_volume.3dm
        ...
```

From here, all user-interfaces can access the data over the GraphQL-API ⚡

# 📊 Stats

<p align="center">
    <a href="https://github.com/usalu/metabolism"><img src="https://myhits.vercel.app/api/hit/https%3A%2F%2Fgithub.com%2Fusalu%2Fmetabolism?color=gray&label=views&size=small" alt="views"/></a>
</p>

![Pixel](https://stats.docs.semio-tech.com/count?p=https://github.com/usalu/metabolism)
