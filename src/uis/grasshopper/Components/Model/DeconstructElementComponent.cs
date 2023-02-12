using System;
using System.Linq;
using Grasshopper.Kernel;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
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
            pManager.AddTextParameter("Sobject Id", "S", "", GH_ParamAccess.item);
            pManager.AddTextParameter("Plan Hash", "H", "", GH_ParamAccess.item);
            pManager.AddParameter(new PoseParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            ElementGoo element = new();
            if (!DA.GetData(0, ref element)) return;
            DA.SetData(0, element.Value.SobjectId);
            DA.SetData(1, element.Value.PrototypePlanHash);
            DA.SetData(2, new PoseGoo(element.Value.Pose));
        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_deconstruct_element;
        public override Guid ComponentGuid => new ("6E1827B1-01B0-4EE2-B56C-9566EAC395F2");

    }
}