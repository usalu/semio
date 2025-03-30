# 3d assests

Reference: https://github.com/pmndrs/assets/tree/main/src/hdri

## Compress hdr

### Imagemagick convert to 512x512 with

From Makefile https://github.com/pmndrs/assets/blob/main/Makefile

```make
%.exr.compressed: %.exr
	convert $< -compress DWAB -resize $(RESIZE) $@
```

https://discourse.threejs.org/t/can-hdr-be-compressed/62964
