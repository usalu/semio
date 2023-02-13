from pytest import mark,fixture

from semio.utils import PointOfViewParser,ViewParser, getLocalPointOfView, getWorldPointOfView

@mark.parametrize('pointOfView,view,point,localPoint,worldPoint',
[
    ([0,0,0],[1.0,0.0,0.0,0.0],[0,0,0],[0,0,0],[0,0,0]),
    ([0,0,0],[1.0,0.0,0.0,0.0],[-520,207,-218],[-520,207,-218],[-520,207,-218]),
    ([240,181,-241],[1.0,0.0,0.0,0.0],[0,0,0],[-240,-181,241],[240,181,-241]),
    ([240,181,-241],[0.3091312646865845,0.6703038215637207,0.43299445509910583,0.5173455476760864],[0,0,0],[162.954544,-129.446609,-324.239777],[240,181,-241]),
    ([240,181,-241],[0.3091312646865845,0.6703038215637207,0.43299445509910583,0.5173455476760864],[-520,207,-218],[-39.316189,-694.76062,-307.517395],[286.868469,-232.35318,-674.261536]),
])
def test_pose_getViews(pointOfView,view,point,localPoint,worldPoint):
    pose = PoseParser(pointOfView,view)
    expectedWorldPoint = pose.getWorldPointOfView(point)
    expectedLocalPoint = pose.getLocalPointOfView(point)
    assert_array_almost_equal(array(expectedWorldPoint),array(worldPoint),decimalPlaces)
    assert_array_almost_equal(array(expectedLocalPoint),array(localPoint),decimalPlaces)
