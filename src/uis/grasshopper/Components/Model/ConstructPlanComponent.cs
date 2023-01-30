using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using Google.Protobuf.Collections;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Components.Model;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructPlanComponent : ConstructComponent
    {
        public ConstructPlanComponent()
          : base("Construct Plan", "Plan", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Url", "U", "", GH_ParamAccess.item);
            pManager.AddParameter(new ParameterParam(),"Parameters", "Pr", "", GH_ParamAccess.list);
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new PlanParam());
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string url = "";
            if (!DA.GetData(0, ref url)) return;

            var parameters = new List<ParameterGoo>();
            DA.GetDataList(2, parameters);

            Plan plan = new Plan()
            {
                Url = url,
            };

            plan.Parameters.AddRange(parameters.Select(x=>x.Value));

            DA.SetData(0, new PlanGoo(plan));
        }
        public override Guid ComponentGuid=>new ("ABA166F3-9E69-4FF2-A400-243FCB26B8E7");
        protected override Bitmap Icon => Resources.icon_construct_parameter;
    }
}