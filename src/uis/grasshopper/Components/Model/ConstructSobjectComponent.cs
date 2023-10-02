using System;
using System.Drawing;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructSobjectComponent : ConstructComponent
    {
        public ConstructSobjectComponent() : base("Construct Sobject", "Sobject", "", "semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new PoseParam());
            pManager[0].Optional = true;
            pManager.AddParameter(new PlanParam(), "Plan", "Pl", "",GH_ParamAccess.item);
            pManager[1].Optional = true;
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new SobjectParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {

            PoseGoo pose = new();
            DA.GetData(0, ref pose);

            PlanGoo plan = new();
            DA.GetData(1, ref plan);

            Sobject sobject = new Sobject()
            {
                Id = Guid.NewGuid().ToString(),
                Pose = pose.Value,
                Plan = plan.Value,
            };

            DA.SetData(0, new SobjectGoo(sobject));
            }
        public override Guid ComponentGuid=> new("6A1FE23A-C11F-43E3-B609-5A50BE7F2EE3");
        protected override Bitmap Icon => Resources.icon_construct_sobject;
    }
}