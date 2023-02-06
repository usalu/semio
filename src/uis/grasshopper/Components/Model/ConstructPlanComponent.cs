﻿using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Components.Model
{
    public class ConstructPlanComponent : ConstructComponent
    {
        public ConstructPlanComponent()
          : base("Construct Plan", "Plan", "", "Semio", "Model")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Uri", "U", "", GH_ParamAccess.item);
            pManager.AddParameter(new ParameterParam(),"Parameters", "Pr", "", GH_ParamAccess.list);
            pManager[1].Optional = true;
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new PlanParam());
            pManager.AddTextParameter("Hash", "H","",GH_ParamAccess.item);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string uri = "";
            if (!DA.GetData(0, ref uri)) return;

            var parameters = new List<ParameterGoo>();
            DA.GetDataList(1, parameters);

            Plan plan = new Plan()
            {
                Uri = uri,
            };

            plan.Parameters.AddRange(parameters.Select(x=>x.Value));


            DA.SetData(0, new PlanGoo(plan));
            DA.SetData(1, Semio.Utils.ToMd5Hash(plan));
        }
        public override Guid ComponentGuid=>new ("ABA166F3-9E69-4FF2-A400-243FCB26B8E7");
        protected override Bitmap Icon => Resources.icon_construct_parameter;
    }
}