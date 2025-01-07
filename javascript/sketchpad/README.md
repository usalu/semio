## sketchpad

A user interface for creating designs.

## Install

`npm install --legacy-peer-deps`

> NOTE: The --legacy-peer-deps is necissary because react-digraph has a dependency on react 16. But according to this [issue](https://github.com/uber/react-digraph/issues/336) it works even on react 18. So far they seemed right.

## Building from source

### Windows

You need to copy semio.ico to build/semio.ico in order for the electron builder to link the icon.
