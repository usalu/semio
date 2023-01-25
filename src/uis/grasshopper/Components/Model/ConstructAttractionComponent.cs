using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
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
    public class ConstructConnectionComponent : ConstructComponent
    {
        public ConstructConnectionComponent() : base("Construct Connection", "Connection", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new ConnectableParam(),"Attractor", "Ar", "", GH_ParamAccess.item);
            pManager.AddParameter(new ConnectableParam(), "Attracted", "Ad", "", GH_ParamAccess.item);
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ConnectionParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //string id = "";
            //if (!DA.GetData(0, ref id)) return;

            ConnectableGoo connecting = new();
            if (!DA.GetData(0, ref connecting)) return;

            ConnectableGoo connected = new();
            if (!DA.GetData(1, ref connected)) return;

            DA.SetData(0, new ConnectionGoo(new Connection()
            {
                Id = Guid.NewGuid().ToString(),
                Attractor = connecting.Value,
                Attracted = connected.Value,
            }));
        }
        public override Guid ComponentGuid => new("2761156E-6A70-4CAC-B50B-42F410C406D2");
        protected override Bitmap Icon => Resources.icon_construct_connection;
    }
}