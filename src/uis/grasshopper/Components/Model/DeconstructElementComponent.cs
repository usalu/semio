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
    public class DeconstructElementComponent : DeconstructComponent
    {
        public DeconstructElementComponent()
          : base("Deconstruct Element", "DeElement", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new ElementParam());
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new PoseParam());
            pManager.AddParameter(new RepresentationParam(), "Representations", "R", "", GH_ParamAccess.list);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            ElementGoo element = new();
            if (!DA.GetData(0, ref element)) return;
            if (element.Value.Pose!=null) DA.SetData(0, new PoseGoo(element.Value.Pose));
            if (element.Value.Representations.Any()) DA.SetDataList(1, element.Value.Representations.Select(e => new RepresentationGoo(e)));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_element;
        public override Guid ComponentGuid => new ("F7D844B8-495D-4A85-A748-4F400DC85183");

    }
}