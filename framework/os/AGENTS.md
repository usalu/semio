# os

os is a cooperative pseudo operating system with version control, collaboration, plugin-mechanisms, renderers and kernels built in.

[s](`s/AGENTS.md`) is one concrete os instance for the design domain.

# App

An app has a manifest, engine, 

# Manifest

A manifest for an app with schema, commands (along with cmd and cde), 

# Schema

A schema for a document with entity definitions.

# EntityDefinition

# Entity

An entity inside a document.

# Engine

A stateful headless computational engine with bidirectional streaming.

The engine maintains a pack buffer for the document. Alternative versions of the document are computed on the fly by materializing it with operations.

# Command

A command send to the engine.

# Protocol

A binary protocol optimized for app native transfer and storage of commands.

# Spk

An spk is a native binary representations for commands.

Protocols are used for communication and storage.

# Cmd

An cmd is a native text representation for commands.

Cdes are used for logging and llms.

# Operation

An operation is a command that modifies a document.

# Patch

A patch is a protocol that modifies a document.

# Op

An op is protocol that modifies a document.

# Document

A document is the data for an app.

# Pack

A pack is a binary representation of a document.

# Dsl

A dsl is a textual representation of a document.

# Space

A space is the ultimate version controlled container for artifacts.

# Artifact

An artifact is 

## Draft

A draft is a volatile artifact.

## Asset

An asset is a persisted artifact (with optional time to live).

# User

## Author

An author is a user with write permission.

## Specator

A spectator is a user with only read permission.

---

```
framework
    <language> # e.g. rs, js, etc for general framework
    module
        math
            <language> # e.g. rs, js, …
        2d
            <language> # e.g. rs, js, …
        3d
            <language> # e.g. rs, js, …
    product
        os
            <language> # e.g. rs, js, etc for general os code
            module
                dsl
                    <language> # e.g. rs, js, …
                vcs
                    <language> # e.g. rs, js, …
                protocol
                    <language> # e.g. rs, js, …
                neural
                    <language> # e.g. rs, js, …
                flow
                    <language> # e.g. rs, js, …
                workflow
                    <language> # e.g. rs, js, …
                …
                renderer
                    <language> e.g. rs, js, …
                        <framework> e.g. react, wpgu, …
                            <main-file> e.g. lib.rs, index.tsx
        server
            …
        presentation
            …
        print
            …
    …
s # os
    plugin
        <plugin> # e.g. puzzle, draw, shooting, procedural, fem, energy, …
            app
                <app>
                    engine
                        rs


                    rs # single crate for the complete
                        engine.rs




                        manifest
                            document.rs
                            pack.rs
                            command
                                <command>.rs # each cd
hub # server
    …
mit-bestand
    …
```