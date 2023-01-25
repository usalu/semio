# from pytest import mark,fixture

# from semio.model import Point,Pose,Quaternion,Sobject,Connection,Connectable,LAYOUTSTRATEGY_BREADTHFIRST,Layout,Any
# from semio.assembler import LayoutDesignRequest, AssemblerProxy

# @fixture
# def sampleLayoutDesignRequest():
#     return LayoutDesignRequest(layout=Layout(
#         sobjects=[
#                 Sobject(id='1',url="elements/RectangleWithMiter.gh",pose=Pose(point_of_view=Point(x=-400,y=10,z=-5),view=Quaternion(w=1,x=0,y=0,z=0)),parameters={'Length':Any(value=b'330')}),
#                 Sobject(id='2',url="elements/RectangleWithMiter.gh",pose=Pose(point_of_view=Point(x=30,y=500,z=20),view=Quaternion(w=0,x=0.707,y=-0.707,z=0)),parameters={'Length':Any(value=b'220')})
#         ],
#         connections=[
#             Connection(id='1',connecting=Connectable(patricipant_id='1'),connected=Connectable(patricipant_id='2'))
#         ],
#         root_sobject_id='1',
#         stragegy=LAYOUTSTRATEGY_BREADTHFIRST
#     ))


# def test_layoutDesign(sampleLayoutDesignRequest):
#     assemblerProxy = AssemblerProxy()
#     response = assemblerProxy.LayoutDesign(request=sampleLayoutDesignRequest)
#     design = response.design
