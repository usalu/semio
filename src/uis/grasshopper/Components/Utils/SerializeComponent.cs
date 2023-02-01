using System;
using System.CodeDom;
using System.Collections.Generic;
using System.Linq;
using Google.Protobuf;
using Grasshopper;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Data;
using Grasshopper.Kernel.Types;
using Rhino;
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
    public class SerializeComponent : GH_Component
    {
        public SerializeComponent()
          : base("Serialize", "Serialize", "", "Semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddGenericParameter("Input","I","",GH_ParamAccess.tree);
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddTextParameter("Serialized","S","",GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            //if (!DA.GetDataTree(0, out GH_Structure<GH_Goo<dynamic>> tree)) return;
            if (!DA.GetDataTree(0, out GH_Structure<IGH_Goo> tree)) return;
            //var obj = new GH_ObjectWrapper();

            //if (!DA.GetData(0, ref obj)) return;
            //if (obj == null) return;

            //dynamic input = ((dynamic)obj.Value).Value;
            DA.SetData(0, Converter.ToString(tree));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_show_design;

        public override Guid ComponentGuid => new ("537857BE-97F2-4A22-BBED-B0C230BA4D8A");

    }
}