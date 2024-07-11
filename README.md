<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/logo/logo-horizontal-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/logo/logo-horizontal.svg">
        <img alt="semio" href="https://github.com/usalu/semio/" src="https://raw.githubusercontent.com/usalu/semio/main/logo/logo-horizontal.svg">
    </picture>
    <br/>
    <a href="https://doi.org/10.5281/zenodo.8419156"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/doizenodo.svg" alt="Cite"></a>
    <a href="https://github.com/usalu/semio/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/latestrelease.svg" alt="Latest Release"></a>
    <a href="https://choosealicense.com/licenses/agpl-3.0/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/licenseagplv3.svg" alt="AGPLv3 License"></a>
    <br/>
    <a href="https://www.grasshopper3d.com/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/uigrasshopper.svg" alt="Grasshopper"></a>
    <a href="https://dynamobim.org/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/uiplanneddynamo.svg" alt="Dynamo"></a>
    <a href="https://nortikin.github.io/sverchok/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/uiplannedsverchok.svg" alt="Sverchok"></a>
    <br/>
    <a href="https://www.python.org/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/poweredbypython.svg" alt="Python"></a>
    <a href="https://learn.microsoft.com/en-us/dotnet/csharp/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/poweredbycsharp.svg" alt="C#"></a>
    <a href="https://www.typescriptlang.org/"><img src="https://raw.githubusercontent.com/usalu/semio/main/docs/badges/poweredbytypescript.svg" alt="Typescript"></a>
    <br/>
    <i>Collaborative, scalable and cross-platform designing.</i>
</p>
<br/>

You want to <emoji>✏️</emoji> the next <emoji>🏛️🏘️🏢🏭🏫🏨⛪🕌</emoji>? But <emoji>📐🔢🗣️</emoji> takes all your <emoji>⌚</emoji>? Then try to capture the <emoji>🧬</emoji> and <emoji>💉</emoji>  it into a new design and let semio <emoji>🖥️✒️🖨️</emoji>.

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/artifacts-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/artifacts.svg">
        <img alt="Capsule Tower Artifacts" src="https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/artifacts.svg">
    </picture>
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/computation-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/computation.svg">
        <img alt="Capsule Tower Computation" src="https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/computation.svg">
    </picture>
</p>

# What is semio?

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/conceptual/analogies-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/conceptual/analogies.svg">
    <img alt="Analogies" src="https://raw.githubusercontent.com/usalu/semio/main/docs/conceptual/analogies.svg">
</picture>

semio is an open ecosystem for component-based design of architecture. You can capture and link design knowledge. Preparing for the next generation of AI. In the mean time, automatic file federation enables collaboration, automation and reuse.

# Getting started

## Requirements
- Windows
- Rhino 7 or greater

semio has currently two user-interfaces. One in Grasshopper and one that is native (sketchpad).

## Installation

The Grasshopper plugin can be installed over the Rhino Package Manager.
1. Open Rhino Package Manger
1. Search for semio
1. Install the latest version
1. Restart Rhino

![Rhino Package](https://raw.githubusercontent.com/usalu/semio/main/docs/installation/rhinopackage.gif)

[sketchpad](https://github.com/usalu/semio/releases/download/r24.07-1/engine.zip) needs no installation. Just unpack the folder and run it. If you run sketchpad without Grasshopper then you need to manually download and start the [engine](https://github.com/usalu/semio/releases/download/r24.07-1/engine.zip).
> NOTE: The binaries are not signed (yet). Therefor Windows might give a warning.

That's it!

## Examples

You can download the examples from the [latest release](https://github.com/usalu/semio/releases/download/r24.07-1/examples.zip).


### Nakagin Capsule Tower

### Requirements
- Rhino model units need to be set to `Meter`.


At least in semio, the iconic capsule tower(s) will keep on existing!
![Nakagin Capsule Tower](https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/grasshopper.gif)

Ever wanted to change the capsules or the shaft with one click? While having different level of details?
![Nakagin Capsule Tower Variants](https://raw.githubusercontent.com/usalu/semio/main/docs/examples/nakagincapsuletower/variants.png)

# How does semio work?

>NOTE: This is outdated. Release r23.07 was a complete rewrite. It is way better now!

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/systemarchitecture/componentsdiagram-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/systemarchitecture/componentsdiagram.svg">
    <img alt="Components" src="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/systemarchitecture/componentsdiagram.svg">
</picture>

semio is an ecosystem of several components. There is a backend that can be extended on different platforms and different frontends for viewing and/or authoring designs. All services including their depenencies are vendored into the platform extensions to run as a binary. Consider the rest as magic.

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/metamodel-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/metamodel.svg">
    <img alt="Metamodel" src="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/metamodel.svg">
</picture>

Here is a brief overview over the process:

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/designprocessmodel-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/designprocessmodel.svg">
    <img alt="Process Part 1" src="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/designprocessmodel.svg">
</picture>

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/designprocessmodel2-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/designprocessmodel2.svg">
    <img alt="Process Part 2" src="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/designprocessmodel2.svg">
</picture>

# Contribution

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/frameworkrelationships-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/frameworkrelationships.svg">
    <img alt="Components" src="https://raw.githubusercontent.com/usalu/semio/main/docs/softwarequality/softwarearchitecture/frameworkrelationships.svg">
</picture>


If you want to contribute to the project, feel free to open an issue. If you are not sure what your contribution could exactly be, feel free to take a look under the [project site](https://github.com/users/usalu/projects/2) and see if find something.