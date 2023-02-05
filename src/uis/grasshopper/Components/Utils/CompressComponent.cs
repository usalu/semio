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
    public class CompressComponent : GH_Component
    {
        public CompressComponent()
          : base("Compress", "Compress", "", "Semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddGenericParameter("Input","I","",GH_ParamAccess.item);
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddTextParameter("Compressed","S","",GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            var obj = new GH_ObjectWrapper();
            if (!DA.GetData(0, ref obj)) return;
            if (!DA.GetData(0, ref obj)) return;
            if (obj == null) return;
            dynamic semioObject = ((dynamic)obj.Value).Value;
            DA.SetData(0, Semio.Utils.Utils.ToBase64(semioObject));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_show_design;
        public override Guid ComponentGuid => new ("3053B0B4-8A3C-4774-9C7A-046472EF8F8A");

    }
}