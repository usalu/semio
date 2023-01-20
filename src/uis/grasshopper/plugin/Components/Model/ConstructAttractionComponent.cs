using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Google.Protobuf.Collections;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using NetMQ.Sockets;
using Rhino.Geometry;
using Semio.Assembler.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructAttractionComponent : ConstructComponent
    {
        public ConstructAttractionComponent() : base("Construct Attraction", "Attraction", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new AttractionParticipantParam(),"Attractor", "Ar", "", GH_ParamAccess.item);
            pManager.AddParameter(new AttractionParticipantParam(), "Attracted", "Ad", "", GH_ParamAccess.item);
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new AttractionParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //string id = "";
            //if (!DA.GetData(0, ref id)) return;

            AttractionParticipantGoo attractor = new();
            if (!DA.GetData(0, ref attractor)) return;

            AttractionParticipantGoo attracted = new();
            if (!DA.GetData(1, ref attracted)) return;

            DA.SetData(0, new AttractionGoo(new Attraction()
            {
                Id = Guid.NewGuid().ToString(),
                Attractor = attractor.Value,
                Attracted = attracted.Value,
            }));
        }
        public override Guid ComponentGuid => new("2761156E-6A70-4CAC-B50B-42F410C406D2");
        protected override Bitmap Icon => Resources.icon_construct_attraction;
    }
}