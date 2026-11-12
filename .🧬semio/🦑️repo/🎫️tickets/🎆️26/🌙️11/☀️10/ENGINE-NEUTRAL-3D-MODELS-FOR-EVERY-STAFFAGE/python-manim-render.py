"""Renders one Scene class of a course clip with the real manim at low quality into the machine cache and prints the movie path.
Usage (workspace root, course environment, SEMIO_TEACHING_LANGUAGE set): python python-manim-render.py <module> <Scene> <media-dir> [--axonometric <cast-id>...]"""
import importlib, sys, os
from pathlib import Path
sys.path.insert(0, str(Path.cwd()))
from manim import config
module, scene, media = sys.argv[1:4]
config.media_dir = media
config.pixel_width, config.pixel_height, config.frame_rate = 640, 360, 15
config.disable_caching = True
config.write_to_movie = True
config.verbosity = "WARNING"
config.progress_bar = "none"
config.quality = "low_quality"
config.pixel_width, config.pixel_height = 640, 360
mod = importlib.import_module(module)
cls = getattr(mod, scene)
instance = cls()
instance.render()
print("MOVIE", config.output_file, instance.renderer.file_writer.movie_file_path)
