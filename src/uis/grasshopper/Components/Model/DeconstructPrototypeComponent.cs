using System;
using System.Collections.Generic;
using System.Linq;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class DeconstructPrototypeComponent : DeconstructComponent
    {
        public DeconstructPrototypeComponent()
          : base("Deconstruct Prototype", "DePrototype", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new PrototypeParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new RepresentationParam(), "Representations", "R", "", GH_ParamAccess.list);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            PrototypeGoo prototype = new();
            if (!DA.GetData(0, ref prototype)) return;
            if (prototype.Value.Representations.Any()) DA.SetDataList(1, prototype.Value.Representations.Select(e => new RepresentationGoo(e)));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_prototype;
        public override Guid ComponentGuid => new ("F7D844B8-495D-4A85-A748-4F400DC85183");

    }
}