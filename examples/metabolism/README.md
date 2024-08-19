# Metabolism

While [metabolists](<https://en.wikipedia.org/wiki/Metabolism_(architecture)>) saw the built environment as an organism that should continuously be able to adapt to the evolving needs of its habitants, I see semio as a metabolistic tool for the evolving needs of a design 🔀

Let me demonstrate metametabolism on metabolism 🤯

## Nakagin Capsule Tower

### Theory ... oh my

There are countless ways to formalize a design ♾️

Assuming I would have been Kisho Kurokawa and used semio, I would chosen the following:

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/artifacts-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/artifacts.svg">
        <img alt="Capsule Tower Artifacts" src="https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/artifacts.svg">
    </picture>
</p>

Based on this formalization the design can be synthesized the following:

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/computation-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/computation.svg">
        <img alt="Capsule Tower Computation" src="https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/computation.svg">
    </picture>
</p>

### Practice ... yey

A bit overwhelmed? Just take a look at the Grasshopper script `nakagin-capule-tower.gh`💡

#### Sketchpad

![Sketchpad Overview](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/sketchpad-overview.png)

#### Grasshopper

> Make sure to open Rhino 8 and set the model units to `Meter`.

After opening the main Grasshopper definition

![Grasshopper Parametric Design](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/grasshopper-definition-main.png)

you will see the following model:

![Rhino Default Variant 1to500 volume](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/rhino-default-variant-1to500-volume.png)

You can easily change the level of detail 🔍

![Rhino Default Variant 1to200 volume](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/rhino-default-variant-1to200-volume.png)

Or the typology 🔁

![Rhino Futuristic Variant 1to500 volume](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/rhino-futuristic-variant-1to500-volume.png)

Or both 🪄

![Rhino Futuristic Variant 1to200 volume](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/rhino-futuristic-variant-1to200-volume.png)

In a conventional block-based approach, block instances have insertion planes and are thus not linked 🪨

Switching from one to another variant would look either like this:

![Block-based Default to Futuristic](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/block-based-default-to-futuristic.png)

or this:

![Block-based Futuristic to Default](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/block-based-futuristic-to-default.png)

Due to the port-mechanism of semio, types are not inserted but instead linked 🔗

Further they have metadata attached:

![Block-based Futuristic to Default](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/rhino-default-variant-metrics.png)

#### GraphQL

If you go to `http://127.0.0.1:5052/graphql/`, you can find all the data that stored and query it:

```graphql
{
  loadLocalKit(directory: "SOMEPARENTDIRECTORY\\examples\\metabolism") {
    kit {
      name
      description
      url
      homepage
      types {
        name
        description
        variant
        unit
        representations {
          url
          lod
          tags
        }
        ports {
          id
          point {
            x
            y
            z
          }
          direction {
            x
            y
            z
          }
          locators {
            group
            subgroup
          }
        }
        qualities {
          name
          value
          unit
          definition
        }
      }
      formations {
        name
        description
        variant
        unit
        pieces {
          id
          type {
            name
            variant
          }
          root {
            plane {
              origin {
                x
                y
                z
              }
              xAxis {
                x
                y
                z
              }
              yAxis {
                x
                y
                z
              }
            }
          }
          diagram {
            point {
              x
              y
            }
          }
        }
        connections {
          connected {
            piece {
              id
              type {
                port {
                  id
                }
              }
            }
          }
          connecting {
            piece {
              id
              type {
                port {
                  id
                }
              }
            }
          }
          offset
          rotation
        }
      }
    }
    error
  }
}
```

and replace `SOMEPARENTDIRECTORY\\examples\\metabolism` with your actual directory path then you should see this:

![GraphQL Query](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/graphql-query.png)

#### Integrations

Looking closely at the `types`, you see that `representation` are not (only) linking to files but instead they link `urls`. This enables you to use any storage, collaboration and version environment you like (Speckle, Dropbox, OneDrive, GitHub, Nextcloud, Spline, ...) or all of them together 🕸️

[Here](https://app.speckle.systems/projects/e7de1a2f8f) you can view the models even on your phone 📱

![Speckle Models Overview](https://raw.githubusercontent.com/usalu/semio/r24.09-1/examples/metabolism/docs/speckle-models-overview.png)

#### Curious, how it works?

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
└───representations
        ...
        capsule_1.3dm
        capsule_1.glb
        capsule_1_1to200_volume.3dm
        capsule_1_1to500_volume.3dm
        ...
```

From here, all user-interfaces can access the data over the GraphQL-API ⚡
