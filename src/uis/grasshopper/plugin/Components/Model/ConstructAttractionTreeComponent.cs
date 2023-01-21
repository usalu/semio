using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using GH_IO.Serialization;
using System.Security.Policy;
using Google.Protobuf.Collections;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Assembler.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructAttractionTreeComponent : ConstructComponent
    {
        public ConstructAttractionTreeComponent() : base("Construct Attraction Tree", "Attraction Tree", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new AttractionParam(),"Attraction", "A", "", GH_ParamAccess.item);
            pManager.AddParameter(new AttractionTreeParam(), "Childrean", "C", "", GH_ParamAccess.list);
            pManager[1].Optional=true;
        }
        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new AttractionTreeParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //string attractionId = "";
            //if (!DA.GetData(0, ref attractionId)) return;

            AttractionGoo attraction = new();
            if (!DA.GetData(0, ref attraction)) return;

            var children = new List<AttractionTreeGoo>();
            DA.GetDataList(1, children);

            AttractionTree attractionTree = new AttractionTree()
            {
                AttractionId = attraction.Value.Id,
            };
            attractionTree.Children.AddRange(children.Select(x=>x.Value));
            DA.SetData(0, new AttractionTreeGoo(attractionTree));
        }
        public override Guid ComponentGuid => new("712F6DA6-F74D-449F-83CE-956255450413");
        protected override Bitmap Icon => Resources.icon_construct_attractiontree;
    }
}