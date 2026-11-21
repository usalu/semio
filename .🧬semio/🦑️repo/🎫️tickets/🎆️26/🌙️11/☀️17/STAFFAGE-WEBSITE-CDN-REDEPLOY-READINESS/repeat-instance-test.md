# Repeated solid tessellation regression

The repeated-solid test compared a world mesh of 2,394 coordinate values to a direct tessellation of 558. World-space instancing uses the current default reference quality of 24 minimum segments. The test used a hard-coded minimum of 12. The two paths were tessellating at different resolutions before the coordinate assertion ran.

The updated test uses the same reference quality for production instancing and direct kernel tessellation. It compares the complete triangle index array and transforms the direct kernel mesh with Three.js to compare every world-space vertex within one nanometre. The instancing test and neighboring fixed-quality triangle test both passed through the repository-pinned Bun, and the Staffage library TypeScript check passed.
