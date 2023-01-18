using System;
using System.Collections.Generic;
using System.Linq;
using Google.Protobuf.Collections;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using NetMQ.Sockets;
using Rhino.Geometry;
using Semio.Gateway.V1;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;

namespace Semio.UI.Grasshopper.Model
{
    public class ConstructSobjectComponent : GH_Component
    {
        /// <summary>
        /// Initializes a new instance of the ConstructSobject class.
        /// </summary>
        public ConstructSobjectComponent()
          : base("Construct Sobject", "Sobject",
              "Construct a sobject",
              "Semio", "Model")
        {
        }

        /// <summary>
        /// Registers all the input parameters for this component.
        /// </summary>
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddTextParameter("Id", "Id", "Identifier of sobject", GH_ParamAccess.item);
            pManager.AddTextParameter("Url", "U", "Url of the element definition.", GH_ParamAccess.item);
            pManager.AddParameter(new PoseParam());
            pManager.AddParameter(new ParameterParam());
        }

        /// <summary>
        /// Registers all the output parameters for this component.
        /// </summary>
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddParameter(new SobjectParam());
        }

        /// <summary>
        /// This is the method that actually does the work.
        /// </summary>
        /// <param name="DA">The DA object is used to retrieve from inputs and store in outputs.</param>
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            string id = "";
            if (!DA.GetData(0, ref id)) return;

            string url = "";
            if (!DA.GetData(1, ref url)) return;

            PoseGoo pose = new();
            if (!DA.GetData(2, ref pose)) return;

            var parameters = new List<ParameterGoo>();
            if (!DA.GetDataList(3, parameters)) return;

            Sobject sobject = new Sobject()
            {
                Id = id,
                Url = url,
                Pose = pose.GetPose()
            };
            sobject.Parameters.Add(new MapField<string, string>
            {
                parameters.ToDictionary(keySelector: m => m.Value.Name, elementSelector: m => m.Value.Value)
            });

            DA.SetData(0, new SobjectGoo(sobject));
        }

        /// <summary>
        /// Provides an Icon for the component.
        /// </summary>
        protected override System.Drawing.Bitmap Icon
        {
            get
            {
                //You can add image files to your project resources and access them like this:
                // return Resources.IconForThisComponent;
                return null;
            }
        }

        /// <summary>
        /// Gets the unique ID for this component. Do not change this ID after release.
        /// </summary>
        public override Guid ComponentGuid
        {
            get { return new Guid("6A1FE23A-C11F-43E3-B609-5A50BE7F2EE3"); }
        }
    }
}