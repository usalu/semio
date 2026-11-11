import importlib
import numpy as np
from manim import VGroup, FadeIn, FadeOut, LaggedStart, Square, tempconfig
e=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':3,'background_color':'#000000'}):
 scene=e.Beat4_Energy(); scene.language=lambda:'en'; scene._prepare()
 counts=scene._counts((0,1)); chips=[VGroup(counts[2*i],counts[2*i+1]) for i in range(4)]
 scene.play(LaggedStart(*[FadeIn(chip) for chip in chips],lag_ratio=.5),run_time=1)
 target=Square(side_length=.4).shift(2.5*np.array([1.,0.,0.]))
 scene.play(FadeOut(counts),FadeOut(*chips),FadeIn(target),run_time=1)
 print('surviving_counts_family',len({id(m) for m in scene.get_mobject_family_members()} & {id(m) for m in counts.get_family()}),flush=True)
 scene.play(FadeIn(Square(side_length=.4).shift(2.5*np.array([-1.,0.,0.]))),run_time=1)
 print('surviving_after_next_play',len({id(m) for m in scene.get_mobject_family_members()} & {id(m) for m in counts.get_family()}),flush=True)
