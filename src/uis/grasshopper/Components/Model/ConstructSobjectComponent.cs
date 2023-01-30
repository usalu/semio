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
    public class ConstructSobjectComponent : ConstructComponent
    {
        public ConstructSobjectComponent() : base("Construct Sobject", "Sobject", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new PoseParam());
            pManager[1].Optional = true;
            pManager.AddParameter(new ParameterParam(), "Parameters", "Pr", "",GH_ParamAccess.list);
            pManager[2].Optional = true;
        }

        protected override void RegisterOutputParams(GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new SobjectParam());
        }

        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string url = "";
            if (!DA.GetData(0, ref url)) return;

            PoseGoo pose = new();
            DA.GetData(1, ref pose);

            PlanGoo plan = new();
            DA.GetData(21, ref plan);

            Sobject sobject = new Sobject()
            {
                Id = Guid.NewGuid().ToString(),
                Pose = pose.GetPose(),
                Plan = plan.Value,
            };

            DA.SetData(0, new SobjectGoo(sobject));
            }
        public override Guid ComponentGuid=> new("6A1FE23A-C11F-43E3-B609-5A50BE7F2EE3");
        protected override Bitmap Icon => Resources.icon_construct_sobject;
    }
}