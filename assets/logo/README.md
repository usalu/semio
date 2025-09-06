# Semio Logo Animation Generator

This Node.js program creates an infinite looping animation from SVG keyframes. The animation loops from keyframes 1 to n, then back from n-1 to 1, creating smooth transitions.

## Features

- **Keyframe-based animation**: Takes multiple SVG files as input keyframes
- **Infinite loop**: Creates smooth forward and reverse animation cycles
- **Transform interpolation**: Extracts and animates translate, rotate, and scale transforms
- **Color animation**: Animates fill and stroke properties
- **Matrix transform support**: Handles both matrix() and individual transform functions

## Usage

```bash
# Install dependencies
npm install

# Generate animated logo from keyframes
npm run animate

# Watch for changes and regenerate
npm run dev
```

## Input Structure

The program expects SVG keyframes with this structure:
- Flat list of `<g>` elements with unique `id` attributes
- Each group has a `transform` attribute (matrix or individual transforms)
- Each group contains one `<path>` element with `d`, `fill`, `stroke`, and `stroke-width` attributes

## Output Structure

The generated animated SVG contains:
- Same group structure with matching IDs
- Three `<animateTransform>` elements per group:
  - `translate` animation
  - `rotate` animation (additive)
  - `scale` animation (additive)
- One `<path>` element per group with:
  - Two `<animate>` elements for `fill` and `stroke` properties

## Example

Given keyframes `logo-1.svg` through `logo-6.svg`, the program generates:
- 10 animation frames total
- Sequence: 1 → 2 → 3 → 4 → 5 → 6 → 5 → 4 → 3 → 2 → (loop)
- 5-second animation duration (0.5 seconds per keyframe)
- Smooth ease-in-out transitions with cubic-bezier easing
- Infinite repeat

## Files

- `logo.ts` - Main animation generator program
- `logo-1.svg` to `logo-6.svg` - Input keyframes
- `logo.svg` - Generated animated output
- `logo-test.html` - Test page for viewing animation
- `logo-old.svg` - Reference implementation

## Transform Parsing

The program parses various transform formats:

```svg
<!-- Matrix transform -->
transform="matrix(2,0,0,-2,20,40)"

<!-- Individual transforms -->
transform="translate(20 40) rotate(90 0 0) scale(2 2)"

<!-- Combined transforms -->
transform="translate(20 40) matrix(-1,0,0,1,0,0)"
```

## Dependencies

- `jsdom` - SVG/XML parsing
- `tsx` - TypeScript execution
- `@types/jsdom` & `@types/node` - TypeScript definitions

## Generated Animation Properties

- **Duration**: 5 seconds per cycle (0.5 seconds per keyframe)
- **Timing**: Smooth ease-in-out transitions between keyframes
- **Easing**: Cubic-bezier (0.42, 0, 0.58, 1) spline interpolation
- **Repeat**: Infinite loop
- **Properties**: transform (translate, rotate, scale), fill, stroke
- **Format**: SVG SMIL animations with calcMode="spline"
