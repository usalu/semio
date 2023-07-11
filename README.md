<p align="center">
    <img src="resources/logo/logo-horizontal.svg" alt="drawing" width="400"/>
</p>
<!-- ![Logo](resources/logo/logo.svg) -->

You want to design the next 🏛️🏘️🏢🏭🏫🏨⛪🕌? But 📐, 🔢 and 🗣️ takes all your time? Then try to capture the 🧬 and 💉 it into a new design and let semio 🖧,🖩,✏️.

<p align="center">
    <img src="docs/conceptual/simplifiedcontext.svg" alt="drawing" width="400"/>
</p>
<!-- ![Conceptual](docs/conceptual/simplifiedcontext.svg) -->

<p align="center">
    <img src="docs/examples/capsuletower/artifacts.svg" alt="drawing" width="800"/>
    <img src="docs/examples/capsuletower/computation.svg" alt="drawing" width="800"/>
</p>
<!-- ![Artifacts](docs/examples/capsuletower/artifacts.svg)
![Computation](docs/examples/capsuletower/computation.svg)
![Variants](docs/examples/capsuletower/variants.png) -->

semio is a framework that makes procedural modelling platforms interoperable. Any file when given to a platform (like Grasshopper, Dynamo, Python, Cadquery, ...) along with paremeters (numerical, textual or geometrical) that returns an output is a script. Depending on the type of inputs and outputs that the script accepts, it can be either a definition, transformation, scheme, modification, factory, stitching or generation.

# Overview

![Components](docs/softwarequality/systemarchitecture/componentsdiagram.svg)

![Example](docs/softwarequality/softwarearchitecture/metamodel.svg)

semio is an ecosystem of several components. There is a backend that can be extended on different platforms and different frontends for viewing and/or authoring designs.

All services can either run locally, with docker-compose, inside kubernetes or be simply called over an active server.

## UIs

Currently there is one UI for Grasshopper.
![Menu ribbon](docs/adapters/grasshopper/ribbon.png)

## Adapters

Currently there is one adapter for Grasshopper that makes it possible to turn Grasshopper scripts into semio scripts.

# Contribution

![Components](docs/softwarequality/softwarearchitecture/frameworkrelationships.svg)


If you want to contribute to the project, there are lot's of opportunities! Do you want to write an apapter for a platform or contribute to the core?
If you are not sure what your contribution could exactly be, feel free to take a look under the [project site](https://github.com/users/usalu/projects/2) and see if find something.