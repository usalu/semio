using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Google.Protobuf.Reflection;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper
{
    public class ConstructScopeComponent : ConstructComponent
    {
        public ConstructScopeComponent()
          : base("Construct Scope", "Scope", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Concept", "Cp", "", GH_ParamAccess.item);
            pManager.AddIntegerParameter("Order", "O", "", GH_ParamAccess.item);
            pManager[1].Optional = true;
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new ScopeParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string concept = "";
            if (!DA.GetData(0, ref concept)) return;

            int order = 0;
            DA.GetData(1, ref order);

            Scope scope = new Scope()
            {
                Concept = concept,
                Order = order
            };

            DA.SetData(0, new ScopeGoo(scope));
        }
        public override Guid ComponentGuid => new("B0AD5DB5-44FB-4501-AB30-9AC1875E4512");
        protected override Bitmap Icon => Resources.icon_construct_scope;
    }
}