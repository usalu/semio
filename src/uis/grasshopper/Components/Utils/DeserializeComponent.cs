using System;
using System.Collections.Generic;
using System.Linq;
using Google.Protobuf;
using Grasshopper;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.FileIO;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;
using Semio.UI.Grasshopper.Utility;
using Speckle.Core.Api;
using Speckle.Core.Models;

namespace Semio.UI.Grasshopper.Utils
{
    public class DeserializeComponent : GH_Component
    {
        public DeserializeComponent()
          : base("Deserialize", "Deserialize", "", "Semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Serialized", "S", "", GH_ParamAccess.item);
            pManager.AddParameter(new PlatformParam());
            pManager[1].Optional = true;
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddGenericParameter("Outputs", "O", "", GH_ParamAccess.list);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string serialized = "";
            if (!DA.GetData(0, ref serialized)) return;

            PlatformGoo platform = new();
            DA.GetData(1, ref platform);

            dynamic outputs;
            switch (platform.Value)
            {
                case Platform.Semio:
                    var output = Operations.Deserialize(serialized);
                    outputs = Converter.Convert(output);
                    break;
                case Platform.Speckle:
                    outputs = Operations.Deserialize(serialized);
                    break;
                default:
                    AddRuntimeMessage(GH_RuntimeMessageLevel.Warning, "This platform is currently not supported.");
                    outputs = "";
                    break;
            }
            DA.SetDataList(0, outputs);
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_show_design;

        public override Guid ComponentGuid => new ("D4C9B4AD-BE63-448A-8C80-501CC8B98C60");

    }
}