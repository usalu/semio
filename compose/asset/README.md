# Summary

Shared assets including badges, fonts, icons, logo, representations, and kit fixtures.

# Docs

## Badges

Each badge is created with [shields.io](https://shields.io) with style `flat-square` and compose colors.

1. Copy the `*.shields` file of an existing badge 📄
1. Open and download the `*.svg` file ⬇

## Fonts

1. Search font on [fontsource.org](https://fontsource.org) 🔍
1. Hit `Download` and extract zip file 📂
1. Use kebaberized font name as folder name and remove everything else (such as version numbers) ➖
1. Merge all types in one folder (`ttf`, `webfonts`, …) - they won't collide due to different extensions 🗃️️
1. Remove all parts that repeat everywhere (such as common name prefix, single weighted fonts, …) 💯

## Icons

1. Open [favicongenerator.net](https://www.favicongenerator.io) 🔍
1. Select `Circle` as `Background Shape` ⏺
1. Select `Anta` as `Font Family` 📃
1. Enter the `Code` that you find in the [dictionary](https://github.com/usalu/semio/tree/main/meta/dictionary.csv)
1. Adjust the `Font Size` to the largest so that the space to the side is the same as the thickness of the stroke 🖊
1. Toggle `Enable SVG` on 🔳
1. Hit `Generate Favicon` and download the zip file to `assets/icon/temp/NAME.zip` where `NAME` is the lowercase name and verb of the icon 📂
1. Repeat the process for all icons 🔁
1. Run `build icons` in the debugger of vscode 🔨

## Kits

`assets/index.ts` is the shared entry point for `compose/asset`. It re-exports the icon layer plus the Metabolism kit fixtures and helper constants. The kit fixtures are available as `MetabolismKit`, `MetabolismKitDiff`, `MetabolismKitDiffed`, `MetabolismKitDiffInverted`, `InvalidKit`, and `InvalidKitValidation`, while each kit entity list is exposed through `MetabolismKitTypes`, `MetabolismKitDesigns`, `MetabolismKitPorts`, `MetabolismKitQualities`, `MetabolismKitFiles`, `MetabolismKitFolders`, `MetabolismKitAuthors`, `MetabolismKitTags`, `MetabolismKitConcepts`, `MetabolismKitAttributes`, and the dedicated `MetabolismKitNakaginCapsuleTowerDesigns`.

Lookup tables `MetabolismKitTypesById`, `MetabolismKitTypesByName`, `MetabolismKitDesignsById`, `MetabolismKitDesignsByName`, `MetabolismKitPortsById`, and `MetabolismKitPortsByName` provide direct access to every type, design, and port without filtering.

# 💯Requirements
