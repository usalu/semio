"""🚘 Verify the actual information-scene driver and shared metre transform."""
import importlib,math,sys
from pathlib import Path
import numpy as np
from manim import Camera,tempconfig
from shapely.geometry import box
from shapely.affinity import rotate,scale
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️")
scene=m.Beat1_DataToInformation.__new__(m.Beat1_DataToInformation)
scene._mid=(0.0,0.0)
scene._c={"car_width":1.75,"car_length":4.60}
out=Path(__file__).parent/"🗑️generated"
with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"background_color":"#000000"}):
    base=scene._car(1,0)
    for heading in (0,90):
        scene._path={"poses":[[0,0,heading],[2,1,heading]],"length":2.236}
        group=scene._driver(1.0)
        car=group[-1]
        expected=m.ORIGIN+m.SCALE*np.array([2,1,0])
        assert np.allclose(car.get_center(),expected)
        reference=scale(rotate(box(-base.width/2,-base.height/2,base.width/2,base.height/2),heading,origin=(0,0)),xfact=m.SCALE,yfact=m.SCALE,origin=(0,0))
        bounds=reference.bounds
        assert math.isclose(car.width,bounds[2]-bounds[0],abs_tol=1e-7)
        assert math.isclose(car.height,bounds[3]-bounds[1],abs_tol=1e-7)
        assert np.allclose(scene._at(2,1)-scene._at(0,0),[.44,.22,0])
        camera=Camera(pixel_width=854,pixel_height=480,background_color="#000000",background_opacity=1)
        camera.capture_mobjects([group])
        camera.get_image().save(out/f"bim114-driver-{heading}.png")
        print("[DEBUG]",heading,"center",car.get_center(),"bounds",car.width,car.height,flush=True)
